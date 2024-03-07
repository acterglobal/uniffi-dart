# TODO Overview

## Introduction
This document provides an overview of the ongoing refactor of this codebase and the remaining features to be implemented. The refactor aims to improve code structure, enhance maintainability, and ensure efficiency. 

In this codebase, rendering Dart language bindings directly from Rust should involve using Rust traits `DartCodeOracle`, `Renderer`, `CodeType`, and `Renderable` to abstractly define the conversion logic for various Rust types and structures into their Dart equivalents. Each Rust component (functions, enums, structs) implements `CodeType` which implements `Renderable` traits to generate Dart syntax, ensuring type-safe, consistent representation across languages without relying on external templates. This method leverages Rust's paradigms, efficiently mapping complex Rust structures to Dart, while maintaining type integrity and language idiomatics through the use of `Renderer` and `DartCodeOracle`.

## Remaining Tasks Overview
- [ ] Futures and Async Dart
- [ ] Callbacks
- [ ] Command Line Interface
- [ ] Collections Types (Maps, Sequences)
- [ ] Other Types (Bytes, Timestamp, Duration, Custom)
- [ ] External crates
- [ ] Fixtures/Tests
- [ ] Memory Optimizations.
- [ ] Better Internal documentation
- [ ] Old Code Removal
- [ ] Refine Old Test to have all types


## Additional Notes
- Members are encouraged to communicate openly about challenges and suggestions.
- Documentation is crucial. Please document all changes and their impact.

## Conclusion
This document serves as a guideline for the ongoing refactor process. It is a dynamic document and should be updated as the refactor progresses.
