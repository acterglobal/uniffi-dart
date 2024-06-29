
impl<'a> DartWrapper<'a> {
    pub fn new(ci: &'a ComponentInterface, config: &'a Config) -> Self {
        let type_renderer = TypeHelpersRenderer::new(config, ci);
        DartWrapper {
            // ci,
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
