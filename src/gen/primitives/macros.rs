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
                        static $type_signature lift(Api api, RustBuffer buf) {
                            return $cl_name.read(api, buf.asUint8List()).value;
                        }
                        static LiftRetVal<$type_signature> read(Api api, Uint8List buf) {
                            return LiftRetVal(buf.buffer.asByteData(buf.offsetInBytes).get$conversion_name(0), $allocation_size);
                        }

                        static RustBuffer lower(Api api, $type_signature value) {
                            final buf = Uint8List($cl_name.allocationSize(value));
                            final byteData = ByteData.sublistView(buf);
                            byteData.set$conversion_name(0, value$endian);
                            return toRustBuffer(api, Uint8List.fromList(buf.toList()));
                        }

                        static int allocationSize([$type_signature value = 0]) {
                          return $allocation_size;
                        }

                        static int write(Api api, $type_signature value, Uint8List buf) {
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
                quote! {
                    class BytesFfiConverter extends FfiConverter<$canonical_name, RustBuffer> {
                        // some use-cases might require big endian byte order,
                        // let's make it handy on all methods
                        @override
                        LiftRetVal<int> read(Api api, Uint8List buf, {Endian endianess = Endian.little}) {
                            ByteData byteData = ByteData.sublistView(buf);
                            int value = byteData.getUint32(0, endianess);
                            return LiftRetVal(value);
                        }

                        @override
                        RustBuffer lower(Api api, int value, {Endian endianess = Endian.little}) {
                            ByteData byteData = ByteData(4);
                            byteData.setUint32(0, value, endianess);
                            return RustBuffer(byteData.buffer.asUint8List());
                        }


                        @override
                        int read(ByteBuffer buf, {Endian endianess = Endian.little}) {
                            ByteData byteData = ByteData.view(buf);
                            return byteData.getUint32(0, endianess);
                        }

                        @override
                        int allocationSize([T value]) {
                            // for 32-bit integers
                            return 4;
                        }

                        @override
                        void write(int value, ByteBuffer buf, {Endian endianess = Endian.little}) {
                            ByteData byteData = ByteData.view(buf);
                            byteData.setUint32(0, value, endianess);
                        }
                    }
                }
            }
        }
    };
}
