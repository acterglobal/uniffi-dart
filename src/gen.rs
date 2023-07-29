use std::collections::HashMap;

use anyhow::{Context, Result};
use camino::Utf8Path;
use camino::Utf8PathBuf;
use genco::fmt;
use genco::prelude::*;
use serde::{Deserialize, Serialize};
// use uniffi_bindgen::MergeWith;
use uniffi_bindgen::{BindingGenerator, BindingsConfig, ComponentInterface};

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

    fn update_from_dependency_configs(&mut self, _config_map: HashMap<&str, &Self>) {}
}

pub struct BindingsGenerator {
    ci: ComponentInterface,
    config: Config,
}

impl BindingsGenerator {
    pub fn new(ci: ComponentInterface, config: Config) -> Self {
        BindingsGenerator { ci, config }
    }
    fn generate(&self) -> dart::Tokens {
        let package_name = self.config.package_name();
        let libname = self.config.cdylib_name();
        quote! {

            library $package_name;

            import "dart:async";
            import "dart:convert";
            import "dart:ffi";
            import "dart:io" show Platform, File, Directory;
            import "dart:isolate";
            import "dart:typed_data";
            import "package:ffi/ffi.dart";

            class UniffiInternalError implements Exception {
                static const int bufferOverflow = 0;
                static const int incompleteData = 1;
                static const int unexpectedOptionalTag = 2;
                static const int unexpectedEnumCase = 3;
                static const int unexpectedNullPointer = 4;
                static const int unexpectedRustCallStatusCode = 5;
                static const int unexpectedRustCallError = 6;
                static const int unexpectedStaleHandle = 7;
                static const int rustPanic = 8;

                final int errorCode;
                final String? panicMessage;

                const UniffiInternalError(this.errorCode, this.panicMessage);

                static UniffiInternalError panicked(String message) {
                return UniffiInternalError(rustPanic, message);
                }

                @override
                String toString() {
                switch (errorCode) {
                    case bufferOverflow:
                    return "UniFfi::BufferOverflow";
                    case incompleteData:
                    return "UniFfi::IncompleteData";
                    case unexpectedOptionalTag:
                    return "UniFfi::UnexpectedOptionalTag";
                    case unexpectedEnumCase:
                    return "UniFfi::UnexpectedEnumCase";
                    case unexpectedNullPointer:
                    return "UniFfi::UnexpectedNullPointer";
                    case unexpectedRustCallStatusCode:
                    return "UniFfi::UnexpectedRustCallStatusCode";
                    case unexpectedRustCallError:
                    return "UniFfi::UnexpectedRustCallError";
                    case unexpectedStaleHandle:
                    return "UniFfi::UnexpectedStaleHandle";
                    case rustPanic:
                    return "UniFfi::rustPanic: $$panicMessage";
                    default:
                    return "UniFfi::UnknownError: $$errorCode";
                }
                }
            }

            const int CALL_SUCCESS = 0;
            const int CALL_ERROR = 1;
            const int CALL_PANIC = 2;

            class RustCallStatus extends Struct {
                @Int8()
                external int code;
                external RustBuffer errorBuf;

                static Pointer<RustCallStatus> allocate({int count = 1}) =>
                calloc<RustCallStatus>(count * sizeOf<RustCallStatus>()).cast();
            }

            T noop<T>(T t) {
                return t;
            }

            T rustCall<T>(Api api, T Function(Pointer<RustCallStatus>) callback) {
                var callStatus = RustCallStatus.allocate();
                final returnValue = callback(callStatus);

                switch (callStatus.ref.code) {
                case CALL_SUCCESS:
                    calloc.free(callStatus);
                    return returnValue;
                case CALL_ERROR:
                    throw callStatus.ref.errorBuf;
                case CALL_PANIC:
                    if (callStatus.ref.errorBuf.len > 0) {
                        final message = liftString(api, callStatus.ref.errorBuf.toIntList());
                        calloc.free(callStatus);
                        throw UniffiInternalError.panicked(message);
                    } else {
                        calloc.free(callStatus);
                        throw UniffiInternalError.panicked("Rust panic");
                    }
                default:
                    throw UniffiInternalError(callStatus.ref.code, null);
                }
            }

            class RustBuffer extends Struct {
                @Int32()
                external int capacity;

                @Int32()
                external int len;

                external Pointer<Uint8> data;

                static RustBuffer fromBytes(Api api, ForeignBytes bytes) {
                    final _fromBytesPtr = api._lookup<
                    NativeFunction<
                        RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_from_bytes().name())));
                    final fromBytes =
                    _fromBytesPtr.asFunction<RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)>();
                    return rustCall(api, (res) => fromBytes(bytes, res));
                }

                void deallocate(Api api) {
                    final _freePtr = api._lookup<
                    NativeFunction<
                        Void Function(RustBuffer, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_free().name())));
                    final free = _freePtr.asFunction<void Function(RustBuffer, Pointer<RustCallStatus>)>();
                    rustCall(api, (res) => free(this, res));
                }

                Uint8List toIntList() {
                    final buf = Uint8List(len);
                    final precast = data.cast<Uint8>();
                    for (int i = 0; i < len; i++) {
                        buf[i] = precast.elementAt(i).value;
                    }
                    return buf;
                }

                @override
                String toString() {
                    String res = "RustBuffer { capacity: $capacity, len: $len, data: $data }";
                    final precast = data.cast<Uint8>();
                    for (int i = 0; i < len; i++) {
                        int char = precast.elementAt(i).value;
                        res += String.fromCharCode(char);
                    }
                    return res;
                }
            }

            String liftString(Api api, Uint8List input) {
                // we have a i32 length at the front
                return utf8.decoder.convert(input);
            }

            Uint8List lowerString(Api api, String input) {
                // FIXME: this is too many memcopies!
                return Utf8Encoder().convert(input);

            }

            RustBuffer toRustBuffer(Api api, Uint8List data) {
                final length = data.length;

                final Pointer<Uint8> frameData = calloc<Uint8>(length); // Allocate a pointer large enough.
                final pointerList = frameData.asTypedList(length); // Create a list that uses our pointer and copy in the data.
                pointerList.setAll(0, data); // FIXME: can we remove this memcopy somehow?

                final bytes = calloc<ForeignBytes>();
                bytes.ref.len = length;
                bytes.ref.data = frameData;
                return RustBuffer.fromBytes(api, bytes.ref);
            }

            T? liftOptional<T>(Api api, Uint8List buf, T Function(Api, Uint8List) lifter) {
                if (buf.isEmpty || buf.first == 0){
                    return null;
                }
                return lifter(api, buf.sublist(5));
            }

            Uint8List lowerOptional<T>(Api api, T? inp, Uint8List Function(Api, T) lowerer) {
                if (inp == null) {
                    final res = Uint8List(1);
                    res.first = 0;
                    return res;
                }
                // converting the inner
                final inner = lowerer(api, inp);
                // preparing the outer
                final offset = 5;
                final res = Uint8List(inner.length + offset);
                // first byte sets the option to as true
                res.setAll(0, [1]);
                // then set the inner size
                final len = Uint32List(1);
                len.first = inner.length;
                res.setAll(1, len.buffer.asUint8List().reversed);
                // then add the actual data
                res.setAll(offset, inner);
                return res;
            }

            class ForeignBytes extends Struct {
                @Int32()
                external int len;

                external Pointer<Uint8> data;
            }
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
        let tokens = BindingsGenerator::new(ci, config).generate();
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
