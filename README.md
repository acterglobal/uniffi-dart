# uniffi-dart

Dart frontend for UniFFI bindings

![License: MIT](https://img.shields.io/github/license/acterglobal/uniffi-dart?style=flat-square) ![Status: experimental](https://img.shields.io/badge/status-experimental-red?style=flat-square)

## Work status

- [x] simple arithmetic
- [ ] primitives
- [x] strings
- [x] rustbuffer
- [ ] rustfuture
- [x] rustcallstatus
- [ ] enums
- [ ] custom objects
  - [ ] object functions
  - [ ] returning custom objects
- [ ] async & futures

## MSRV: 1.65

This project must always work on latest stable rust + version before. We are also testing it against 1.65.0 , which we consider the Minimum Support Rust Version (MSRV) at this point. Rust lower than that will probably not compile the project.

## License & Credits

The code is released under MIT License. See the LICENSE file in the repository root for details.

The project is building on top of the great work of Mozillas UniFFI, with inspirations from other external frontends (like Kotlin and Go) and with the help of the [ffi-gen](https://github.com/acterglobal/ffi-gen) lib. Thanks folks!
