use std::collections::HashMap;

use anyhow::Result;
use camino::Utf8Path;

use genco::fmt;
use genco::prelude::*;
use serde::{Deserialize, Serialize};
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

        fn uniffi_function_definitions(ci: &ComponentInterface) -> dart::Tokens {
            let mut definitions = quote!();

            for fun in ci.iter_ffi_function_definitions() {
                let fun_name = fun.name();
                let (native_return_type, dart_return_type) = match fun.return_type() {
                    Some(return_type) => (
                        quote! { $(oracle::DartCodeOracle::ffi_native_type_label(Some(&return_type))) },
                        quote! { $(oracle::DartCodeOracle::ffi_dart_type_label(Some(&return_type))) },
                    ),
                    None => (quote! { Void }, quote! { void }),
                };

                let (native_args, dart_args) = {
                    let mut native_args = quote!();
                    let mut dart_args = quote!();

                    for arg in fun.arguments() {
                        native_args.append(
                            quote!($(oracle::DartCodeOracle::ffi_native_type_label(Some(&arg.type_()))),),
                        );
                        dart_args.append(
                            quote!($(oracle::DartCodeOracle::ffi_dart_type_label(Some(&arg.type_()))),),
                        );
                    }

                    if fun.has_rust_call_status_arg() {
                        native_args.append(quote!(Pointer<RustCallStatus>));
                        dart_args.append(quote!(Pointer<RustCallStatus>));
                    }

                    (native_args, dart_args)
                };

                let lookup_fn = quote! {
                    _dylib.lookupFunction
                        $native_return_type Function($(&native_args)),
                        $(&dart_return_type) Function($(&dart_args))
                    >($(format!("\"{fun_name}\"")))
                };

                definitions.append(quote! {
                    late final $dart_return_type Function($dart_args) $fun_name = $lookup_fn;
                });
            }

            definitions
        }

        quote! {
            library $package_name;

            $(type_helper_code) // Imports, Types and Type Helper

            $(functions_definitions)

            class _UniffiLib {
                _UniffiLib._();

                static final DynamicLibrary _dylib = _open();

                static DynamicLibrary _open() {
                  if (Platform.isAndroid) return DynamicLibrary.open($(format!("\"lib{libname}.so\"")));
                  if (Platform.isIOS) return DynamicLibrary.executable();
                  if (Platform.isLinux) return DynamicLibrary.open($(format!("\"lib{libname}.so\"")));
                  if (Platform.isMacOS) return DynamicLibrary.open($(format!("\"lib{libname}.dylib\"")));
                  if (Platform.isWindows) return DynamicLibrary.open($(format!("\"{libname}.dll\"")));
                  throw UnsupportedError("Unsupported platform: ${Platform.operatingSystem}");
                }

                static final _UniffiLib instance = _UniffiLib._();

                $(uniffi_function_definitions(self.ci))

                static void _checkApiVersion() {
                    final bindingsVersion = $(self.ci.uniffi_contract_version());
                    final scaffoldingVersion = _UniffiLib.instance.$(self.ci.ffi_uniffi_contract_version().name())();
                    if (bindingsVersion != scaffoldingVersion) {
                      throw UniffiInternalError.panicked("UniFFI contract version mismatch: bindings version $bindingsVersion, scaffolding version $scaffoldingVersion");
                    }
                }

                static void _checkApiChecksums() {
                    $(for (name, expected_checksum) in self.ci.iter_checksums() =>
                        if (_UniffiLib.instance.$(name)() != $expected_checksum) {
                          throw UniffiInternalError.panicked("UniFFI API checksum mismatch");
                        }
                    )
                }
            }

            void initialize() {
                _UniffiLib._open();
            }

            void ensureInitialized() {
                _UniffiLib._checkApiVersion();
                _UniffiLib._checkApiChecksums();
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
        _try_format_code: bool,
    ) -> Result<()> {
        let filename = out_dir.join(format!("{}.dart", config.cdylib_name()));
        let tokens = DartWrapper::new(ci, config).generate();
        let file = std::fs::File::create(filename)?;

        let mut w = fmt::IoWriter::new(file);

        let fmt = fmt::Config::from_lang::<Dart>().with_indentation(fmt::Indentation::Space(4));
        let config = dart::Config::default();

        tokens.format_file(&mut w.as_formatter(&fmt), &config)?;
        Ok(())
    }
    fn check_library_path(
        &self,
        _library_path: &Utf8Path,
        _cdylib_name: Option<&str>,
    ) -> Result<()> {
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
        &DartBindingGenerator {},
        udl_file,
        config_file_override,
        out_dir_override,
        library_file,
        None,
        true,
    )
}

