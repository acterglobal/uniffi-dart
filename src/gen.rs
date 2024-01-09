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

mod render;
mod enums;
mod functions;
mod objects;
mod primitives;
mod records;
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
    // fn get_entry_from_bindings_table(_bindings: &Value) -> Option<Value> {
    //     if let Some(table) = _bindings.as_table() {
    //         table.get("dart").map(|v| v.clone())
    //     } else {
    //         None
    //     }
    // }

    // fn update_from_ci(ci: &ComponentInterface) -> Vec<(String, Value)> {
    //     vec![
    //         (
    //             "package_name".to_string(),
    //             Value::String(ci.namespace().to_string()),
    //         ),
    //         (
    //             "cdylib_name".to_string(),
    //             Value::String(ci.namespace().to_string()),
    //         ),
    //     ]
    // }
    const TOML_KEY: &'static str = "dart";

    fn update_from_ci(&mut self, ci: &ComponentInterface) {
        self.cdylib_name
            .get_or_insert_with(|| format!("uniffi_{}", ci.namespace()));
    }

    fn update_from_cdylib_name(&mut self, cdylib_name: &str) {
        self.cdylib_name
            .get_or_insert_with(|| cdylib_name.to_string());
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
    type_helper_code: dart::Tokens,
}

impl<'a> DartWrapper<'a> {
    pub fn new(ci: &'a ComponentInterface, config: &'a Config) -> Self {
        let type_renderer = TypeHelpersRenderer::new(config, ci);
        DartWrapper { ci, config, type_helper_code: type_renderer.render(&type_renderer) }
    }

    fn generate(&self) -> dart::Tokens {
        let package_name = self.config.package_name();
        let libname = self.config.cdylib_name();
        quote! {
            library $package_name;

            $(&self.type_helper_code) // Imports and type conversion code
            
            $( for rec in self.ci.record_definitions() => $(records::generate_record(rec)))

            $( for enm in self.ci.enum_definitions() => $(enums::generate_enum(enm)))
            $( for obj in self.ci.object_definitions() => $(objects::generate_object(obj)))

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

                $( for fun in self.ci.function_definitions() => $(functions::generate_function("this", fun)))
            }
        }
    }
}

pub struct DartBindingGenerator;

impl BindingGenerator for DartBindingGenerator {
    type Config = Config;

    fn write_bindings(
        &self,
        ci: ComponentInterface,
        config: Self::Config,
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
}

fn parse_udl(udl_file: &Utf8Path) -> Result<ComponentInterface> {
    let udl = std::fs::read_to_string(udl_file)
        .with_context(|| format!("Failed to read UDL from {udl_file}"))?;
    ComponentInterface::from_webidl(&udl).context("Failed to parse UDL")
}

fn get_config(
    ci: &ComponentInterface,
    crate_root: &Utf8Path,
    config_file_override: Option<&Utf8Path>,
) -> Result<Config> {
    let default_config: Config = ci.into();

    let config_file = match config_file_override {
        Some(cfg) => Some(cfg.to_owned()),
        None => crate_root.join("uniffi.toml").canonicalize_utf8().ok(),
    };

    match config_file {
        Some(path) => {
            let contents = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read config file from {path}"))?;
            let mut loaded_config: Config = toml::de::from_str(&contents)
                .with_context(|| format!("Failed to generate config from file {path}"))?;
            loaded_config.update_from_ci(&ci);
            Ok(loaded_config)
        }
        None => Ok(default_config),
    }
}

fn get_out_dir(udl_file: &Utf8Path, out_dir_override: Option<&Utf8Path>) -> Result<Utf8PathBuf> {
    Ok(match out_dir_override {
        Some(s) => {
            // Create the directory if it doesn't exist yet.
            std::fs::create_dir_all(s)?;
            s.canonicalize_utf8().context("Unable to find out-dir")?
        }
        None => udl_file
            .parent()
            .context("File has no parent directory")?
            .to_owned(),
    })
}

pub fn generate_dart_bindings(
    udl_file: &Utf8Path,
    config_file_override: Option<&Utf8Path>,
    out_dir_override: Option<&Utf8Path>,
    library_file: Option<&Utf8Path>,
) -> Result<()> {
    let mut component = parse_udl(udl_file)?;
    if let Some(library_file) = library_file {
        uniffi_bindgen::macro_metadata::add_to_ci_from_library(&mut component, library_file)?;
    }
    let crate_root = &uniffi_bindgen::guess_crate_root(udl_file)?;

    let config = get_config(&component, crate_root, config_file_override)?;
    let out_dir = get_out_dir(udl_file, out_dir_override)?;
    DartBindingGenerator.write_bindings(component, config, &out_dir)
}
