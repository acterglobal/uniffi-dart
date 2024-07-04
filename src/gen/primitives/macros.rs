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

                fn ffi_converter_name(&self) -> String {
                    format!("FfiConverter{}", self.canonical_name())
                }

                // The following must create an instance of the converter object
                fn lower(&self) -> String {
                    format!("{}.lower", self.ffi_converter_name())
                }

                fn write(&self) -> String {
                    format!("{}.write", self.ffi_converter_name())
                }

                fn lift(&self) -> String {
                    format!("{}.lift", self.ffi_converter_name())
                }

                fn read(&self) -> String {
                    format!("{}.read", self.ffi_converter_name())
                }
            }
        }
    };
}

macro_rules! impl_renderable_for_primitive {
    ($T:ty, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                use uniffi_bindgen::backend::CodeType;
                let endian = (if $canonical_name.contains("Float") {
                    ", Endian.little"
                } else {
                    ""
                });

                let cl_name = &self.ffi_converter_name();
                let type_signature = &self.type_label();
                let conversion_name = &$canonical_name
                                    .replace("UInt", "Uint")
                                    .replace("Double", "Float");

                quote! {
                    class $cl_name {
                        // static $type_signature lift($type_signature value) {
                        //     return $cl_name.read(buf.asUint8List()).value;
                        // }
                        // According to generated funtion signatures, we won't need to convert number types
                        static $type_signature lift($type_signature value) => value;


                        static LiftRetVal<$type_signature> read(Uint8List buf) {
                            return LiftRetVal(buf.buffer.asByteData(buf.offsetInBytes).get$conversion_name(0), $allocation_size);
                        }

                        // static RustBuffer lower($type_signature value) {
                        //     final buf = Uint8List($cl_name.allocationSize(value));
                        //     final byteData = ByteData.sublistView(buf);
                        //     byteData.set$conversion_name(0, value$endian);
                        //     return toRustBuffer(Uint8List.fromList(buf.toList()));
                        // }

                        static $type_signature lower($type_signature value) => value;


                        static int allocationSize([$type_signature value = 0]) {
                          return $allocation_size;
                        }

                        static int write($type_signature value, Uint8List buf) {
                            buf.buffer.asByteData(buf.offsetInBytes).set$conversion_name(0, value$endian);
                            return $cl_name.allocationSize();
                        }

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
                        LiftRetVal<int> read(Uint8List buf) {
                            // final uint_list = buf.toIntList();
                            // return uint_list.buffer.asByteData().get$canonical_name(1);
                        }

                        @override
                        RustBuffer lower(int value) {
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

