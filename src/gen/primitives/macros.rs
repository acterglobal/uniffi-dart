use genco::prelude::*;
use paste::paste;

macro_rules! impl_code_type_for_primitive {
    ($T:ty, $class_name:literal, $canonical_name:literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T;

            impl uniffi_bindgen::backend::CodeType for $T  {
                fn type_label(&self,) -> String {
                    $class_name.into()
                }

                fn literal(&self, literal: &uniffi_bindgen::backend::Literal) -> String {
                    $crate::gen::primitives::render_literal(&literal)
                }

                fn canonical_name(&self,) -> String {
                    $canonical_name.into()
                }
            }
        }
    };
}

macro_rules! impl_renderable_for_primitive {
    ($T:ty, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                // TODO: Need to modify behavior to allow
                // if (type_helper.check($canonical_name)) {
                //     return quote!()
                // }
                // This method can be expanded to generate type helper methods if needed.
                let mut endian = (if $canonical_name.contains("Float") {
                    "Endian.little"
                } else {
                    "Endian.big"
                });
                let _final_uintlist = (if $canonical_name.contains("Float") {
                    String::from($canonical_name) + "List.fromList(buf.reversed.toList())"
                } else {
                    String::from($canonical_name) + "List.fromList(buf.toList())"
                });

                let cl_name = &format!("FfiConverter{}", $canonical_name);
                let data_type = &$canonical_name
                    .replace("UInt", "Uint")
                    .replace("Double", "Float");
                let type_signature = if data_type.contains("Float") {
                    "double"
                } else {
                    endian = "";
                    "int"
                };

                quote! {
                    class $cl_name {
                        static LiftRetVal<$type_signature> read(Api api, Uint8List buf) {
                            return LiftRetVal(buf.buffer.asByteData().get$data_type(0), $allocation_size);
                        }

                        static RustBuffer lower(Api api, $type_signature value) {
                            final buf = Uint8List($cl_name.allocationSize());
                            final byteData = ByteData.sublistView(buf);
                            byteData.set$data_type(0, value, $endian);
                            return toRustBuffer(api, Uint8List.fromList(buf.toList()));
                        }

                        static int allocationSize([$type_signature value = 0]) {
                          return $allocation_size;
                        }

                        // @override
                        // void write($type_signature value, ByteBuffer buf) {
                        //     throw UnimplementedError("Should probably implement writes now");
                        // }
                    }
                }
            }
        }
    };

    (BytesCodeType, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                if (type_helper.check($canonical_name)) {
                    return quote!(); // Return an empty string to avoid code duplication
                }
                // TODO: implement bytes ffi methods
                quote! {
                    class BytesFfiConverter extends FfiConverter<$canonical_name, RustBuffer> {
                        @override
                        LiftRetVal<int> read(Api api, Uint8List buf) {
                            // final uint_list = buf.toIntList();
                            // return uint_list.buffer.asByteData().get$canonical_name(1);
                        }

                        @override
                        RustBuffer lower(Api api, int value) {
                            // final uint_list = Uint8List.fromList([value ? 1 : 0]);
                            // final byteData = ByteData.sublistView(buf);
                            // byteData.setInt16(0, value, Endian.little);
                            // return buf;
                        }

                        @override
                        int read(ByteBuffer buf) {
                        //     // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                        //     // performance reasons
                        //   throw UnimplementedError("Should probably implement read now");
                        }

                        @override
                        int allocationSize([T value]) {
                        //   return $allocation_size; // 1 = 8bits//TODO: Add correct allocation size for bytes, change the arugment type
                        }

                        @override
                        void write(int value, ByteBuffer buf) {
                            // throw UnimplementedError("Should probably implement read now");
                        }
                    }
                }
            }
        }
    };
}
