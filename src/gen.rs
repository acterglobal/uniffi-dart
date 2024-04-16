use std::collections::HashMap;

use anyhow::{Context, Result};
use camino::Utf8Path;
use camino::Utf8PathBuf;
use genco::fmt;
use genco::prelude::*;
use serde::{Deserialize, Serialize};
// use uniffi_bindgen::MergeWith;
use uniffi_bindgen::{BindingGenerator, BindingsConfig, ComponentInterface};

use self::render::Renderer;
use self::types::TypeHelpersRenderer;

mod compounds;
mod enums;
mod functions;
mod objects;
mod oracle;
mod primitives;
mod records;
mod render;
mod types;
mod utils;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    package_name: Option<String>,
    cdylib_name: Option<String>,
    #[serde(default)]
    external_packages: HashMap<String, String>,
}

// impl MergeWith for Config {
//     fn merge_with(&self, other: &Self) -> Self {
//         let package_name = if other.package_name.is_some() {
//             other.package_name.clone()
//         } else {
//             self.package_name.clone()
//         };
//         let cdylib_name = if other.cdylib_name.is_some() {
//             other.cdylib_name.clone()
//         } else {
//             self.cdylib_name.clone()
//         };
//         Self {
//             package_name,
//             cdylib_name,
//         }
//     }
// }

impl From<&ComponentInterface> for Config {
    fn from(ci: &ComponentInterface) -> Self {
        Config {
            package_name: Some(ci.namespace().to_owned()),
            cdylib_name: Some(ci.namespace().to_owned()),
            external_packages: HashMap::new(),
        }
    }
}

impl Config {
    pub fn package_name(&self) -> String {
        if let Some(package_name) = &self.package_name {
            package_name.clone()
        } else {
            "uniffi".into()
        }
    }

    pub fn cdylib_name(&self) -> String {
        if let Some(cdylib_name) = &self.cdylib_name {
            cdylib_name.clone()
        } else {
            "uniffi".into()
        }
    }
}

impl BindingsConfig for Config {
    fn update_from_ci(&mut self, ci: &ComponentInterface) {
        self.package_name = Some(ci.namespace().to_owned());
    }

    fn update_from_cdylib_name(&mut self, cdylib_name: &str) {
        self.cdylib_name = Some(cdylib_name.to_string());
    }

    fn update_from_dependency_configs(&mut self, config_map: HashMap<&str, &Self>) {
        for (crate_name, config) in config_map {
            if !self.external_packages.contains_key(crate_name) {
                self.external_packages
                    .insert(crate_name.to_string(), config.package_name());
            }
        }
    }
}

pub struct DartWrapper<'a> {
    config: &'a Config,
    ci: &'a ComponentInterface,
    type_renderer: TypeHelpersRenderer<'a>,
}

impl<'a> DartWrapper<'a> {
    pub fn new(ci: &'a ComponentInterface, config: &'a Config) -> Self {
        let type_renderer = TypeHelpersRenderer::new(config, ci);
        DartWrapper {
            ci,
            config,
            type_renderer,
        }
    }

    fn generate(&self) -> dart::Tokens {
        let package_name = self.config.package_name();
        let libname = self.config.cdylib_name();

        let (type_helper_code, functions_definitions) = &self.type_renderer.render();

        quote! {
            library $package_name;

            $(type_helper_code) // Imports, Types and Type Helper

            class Api {
                final Pointer<T> Function<T extends NativeType>(String symbolName)
                    _lookup;

                Api(DynamicLibrary dynamicLibrary)
                    : _lookup = dynamicLibrary.lookup;

                Api.fromLookup(
                    Pointer<T> Function<T extends NativeType>(String symbolName)
                        lookup)
                    : _lookup = lookup;

                factory Api.loadStatic() {
                    return Api(DynamicLibrary.executable());
                }

                factory Api.loadDynamic(String name) {
                    return Api(DynamicLibrary.open(name));
                }

                factory Api.load() {
                    String? name;
                    if (Platform.isLinux) name = $(format!("\"lib{libname}.so\""));
                    if (Platform.isAndroid) name = $(format!("\"lib{libname}.so\""));
                    if (Platform.isMacOS) name = $(format!("\"lib{libname}.dylib\""));
                    if (Platform.isIOS) name = "";
                    if (Platform.isWindows) name = $(format!("\"{libname}.dll\""));
                    if (name == null) {
                        throw UnsupportedError("This platform is not supported.");
                    }
                    if (name == "") {
                        return Api.loadStatic();
                    } else {
                        return Api.loadDynamic(name);
                    }
                }

                $(functions_definitions)
            }
        }
    }
}

pub struct DartBindingGenerator;

impl BindingGenerator for DartBindingGenerator {
    type Config = Config;

    fn write_bindings(
        &self,
        ci: &ComponentInterface,
        config: &Self::Config,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        let filename = out_dir.join(format!("{}.dart", config.cdylib_name()));
        let tokens = DartWrapper::new(&ci, &config).generate();
        let file = std::fs::File::create(filename)?;

        let mut w = fmt::IoWriter::new(file);

        let fmt = fmt::Config::from_lang::<Dart>().with_indentation(fmt::Indentation::Space(4));
        let config = dart::Config::default();

        tokens.format_file(&mut w.as_formatter(&fmt), &config)?;
        Ok(())
    }
    fn check_library_path(&self, library_path: &Utf8Path, cdylib_name: Option<&str>) -> Result<()> {
        // FIXME: not sure what to check for here...?
        Ok(())
    }
}

pub fn generate_dart_bindings(
    udl_file: &Utf8Path,
    config_file_override: Option<&Utf8Path>,
    out_dir_override: Option<&Utf8Path>,
    library_file: Option<&Utf8Path>,
) -> Result<()> {
    uniffi_bindgen::generate_external_bindings(
        DartBindingGenerator {},
        udl_file,
        config_file_override,
        out_dir_override,
        library_file,
        None,
    )
}
