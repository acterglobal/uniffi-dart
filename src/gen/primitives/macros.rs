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

                let cl_name =  format!("FfiConverter{}", $canonical_name);
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
                    class $cl_name extends FfiConverter<$type_signature, $type_signature> 
                    with FfiConverterPrimitive<$type_signature> {
                        @override
                        $type_signature read(ByteData buffer, int offset) => buffer.get$data_type(offset);

                        @override
                        int size([$type_signature value = $allocation_size]) {
                          return $allocation_size;
                        }

                        @override
                        void write($type_signature value, ByteData buffer, int offset) =>
                        buffer.set$data_type(offset, value);
                    }
                }
            }
        }
    };

    (BooleanCodeType) => {
        impl Renderable for BooleanCodeType {
            fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                // if (type_helper.check($canonical_name)) {
                //     return quote!()
                // }
                // This method can be expanded to generate type helper methods if needed.
                quote! {
                    class FfiConverterBool implements FfiConverter<bool, int> {
                        const FfiConverterBool();
        
                        @override
                        bool lift(int value) => value != 0;
        
                        @override
                        int lower(bool value) => value ? 1 : 0;
        
                        @override
                        bool read(ByteData buffer, int offset) => buffer.getInt8(offset) != 0;
        
                        @override
                        void write(bool value, ByteData buffer, int offset) {
                        buffer.setInt8(offset, lower(value));
                        }
        
                        @override
                        int size(value) => 1;
                    }
                }
            }
        }
    };

    (StringCodeType) => {
        impl Renderable for StringCodeType {
            fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                // This method can be expanded to generate type helper methods if needed.
                quote! {
                    // if (type_helper.check($canonical_name)) {
                    //     return quote!()
                    // }
                    class FfiConverterString implements FfiConverter<String, RustBuffer> {
                        const FfiConverterString();
                        // TODO: Figure out why there's spooky behavior here, default should be four, will fix later
                        String lift(RustBuffer value, [int offset = 0]) {
                            try {
                                final data = value.asTypedList().buffer.asUint8List(offset);
                                return utf8.decoder.convert(data);
                            } finally {
                                value.free();
                            }
                        }
        
                        @override
                        RustBuffer lower(String value) {
                            final buffer = toRustBuffer(Utf8Encoder().convert(value)); // TODO: Fix the meme copies issue by first fixing read
                            return buffer;
                        }
        
                        @override
                        String read(ByteData buffer, int offset) {
                            // TODO! : Fix this, it shouldn't append the lenth to every string, please remove first four bytes later
                            final length = buffer.getInt32(offset);
                            final stringBytes = buffer.buffer.asUint8List(offset + 4, length);
                            return utf8.decoder.convert(stringBytes);
                        }
        
                        @override
                        void write(String value, ByteData buffer, int offset) {
                        final stringBytes = utf8.encode(value);
                        buffer.setInt32(offset, stringBytes.length);
                        buffer.buffer.asUint8List(offset + 4).setAll(0, stringBytes);
                        }
        
                        @override
                        int size(value) => 4 + utf8.encode(value).length;
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
                        int lift(RustBuffer buf, [int offset = 0]) {
                            // final uint_list = buf.asUint8List();
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
                        int size([T value]) {
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