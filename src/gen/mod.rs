use std::collections::HashMap;

use anyhow::Result;
use camino::Utf8Path;

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
    // ci: &'a ComponentInterface,
    type_renderer: TypeHelpersRenderer<'a>,
}

impl<'a> DartWrapper<'a> {
    // ... (previous methods remain the same)

    fn generate(&self) -> dart::Tokens {
        let package_name = self.config.package_name();
        let libname = self.config.cdylib_name();

        let (type_helper_code, functions_definitions) = &self.type_renderer.render();

        quote! {
            library $package_name;

            import 'dart:async';
            import 'dart:ffi';
            import 'dart:typed_data';
            import 'package:ffi/ffi.dart';

            $(type_helper_code) // Imports, Types and Type Helper

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

                $(functions_definitions)

                // ... (FFI function definitions)
            }

            void initialize() {
                _UniffiLib._open();
            }

            $(self.generate_helper_functions())
        }
    }

    fn generate_helper_functions(&self) -> dart::Tokens {
        quote! {
            T rustCall<T>(T Function(Pointer<RustCallStatus>) callback) {
                final status = calloc<RustCallStatus>();
                try {
                    final result = callback(status);
                    final code = status.ref.code;
                    switch (code) {
                        case 0:  // UNIFFI_SUCCESS
                            return result;
                        case 1:  // UNIFFI_ERROR
                            throw status.ref.errorBuf.consumeIntoString();
                        case 2:  // UNIFFI_PANIC
                            final message = status.ref.errorBuf.consumeIntoString();
                            throw UniffiInternalError.panicked(message);
                        default:
                            throw UniffiInternalError.unknownCodec(code);
                    }
                } finally {
                    calloc.free(status);
                }
            }

            Future<T> uniffiRustCallAsync<T, F>(
                int Function() rustFutureFunc,
                void Function(int, Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>, int) pollFunc,
                F Function(int, Pointer<RustCallStatus>) completeFunc,
                void Function(int) freeFunc,
                T Function(F) liftFunc, [
                UniffiRustCallStatusErrorHandler? errorHandler,
            ]) async {
                final rustFuture = rustFutureFunc();
                final completer = Completer<int>();

                late final NativeCallable<UniffiRustFutureContinuationCallback> callback;

                void poll() {
                    pollFunc(
                        rustFuture,
                        callback.nativeFunction,
                        0,
                    );
                }
                void onResponse(int _idx, int pollResult) {
                    if (pollResult == 0) {  // UNIFFI_POLL_READY
                        completer.complete(pollResult);
                    } else {
                        poll();
                    }
                }
                callback = NativeCallable<UniffiRustFutureContinuationCallback>.listener(onResponse);

                try {
                    poll();
                    await completer.future;
                    callback.close();

                    final status = calloc<RustCallStatus>();
                    try {
                        final result = completeFunc(rustFuture, status);
                        final errorHandler = UniffiRustCallStatusErrorHandler();
                        errorHandler.checkStatus(status.ref);
                        return liftFunc(result);
                    } finally {
                        calloc.free(status);
                    }
                } finally {
                    freeFunc(rustFuture);
                }
            }
        }
    }
}

// ... (rest of the file remains the same)

