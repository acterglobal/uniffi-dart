# uniffi-dart

Dart frontend for UniFFI bindings

![License: MIT](https://img.shields.io/github/license/acterglobal/uniffi-dart?style=flat-square) ![Status: experimental](https://img.shields.io/badge/status-experimental-red?style=flat-square)

## Work status

Reference: [TODOs](./TODO.md)

## MSRV: 1.75

This project must always work on latest stable rust + version before. We are also testing it against 1.1.70.0 , which we consider the Minimum Support Rust Version (MSRV) at this point. Rust lower than that will probably not compile the project.

## Integration Tests

The original command is the following:

```
cargo nextest run --all --nocapture
```

If you want to test only the specific module, please use the following command:

```
cargo nextest run -p hello_world --nocapture
```

`genco` is based on `proc_macro_span`, so if you want fully functional whitespace detection, you must build and run projects using `genco` with a nightly compiler until `proc-macro2` is stablized.

```
cargo +nightly nextest run -p hello_world --nocapture
```

## License & Credits

The code is released under MIT License. See the LICENSE file in the repository root for details.

The project is building on top of the great work of Mozillas UniFFI, with inspirations from other external frontends (like Kotlin and Go) and with the help of the [ffi-gen](https://github.com/acterglobal/ffi-gen) lib. Thanks folks!
