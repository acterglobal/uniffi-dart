use genco::lang::dart;
use genco::prelude::*;
use paste::paste;
use uniffi_bindgen::backend::CodeType;
use uniffi_bindgen::interface::{Literal, Type};

use super::oracle::{AsCodeType, DartCodeOracle};
use crate::gen::render::{Renderable, TypeHelperRenderer};

fn render_literal(literal: &Literal, inner: &Type) -> String {
    match literal {
        Literal::None => "null".into(),
        Literal::EmptySequence => "[]".into(),
        Literal::EmptyMap => "{}".into(),

        // For optionals
        _ => DartCodeOracle::find(inner).literal(literal),
    }
}

macro_rules! impl_code_type_for_compound {
     ($T:ty, $type_label_pattern:literal, $canonical_name_pattern: literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T {
                self_type: Type,
                inner: Type,
            }

            impl $T {
                pub fn new(self_type: Type, inner: Type) -> Self {
                    Self { self_type, inner }
                }
                fn inner(&self) -> &Type {
                    &self.inner
                }
            }

            impl CodeType for $T  {
                fn type_label(&self) -> String {
                    format!($type_label_pattern, DartCodeOracle::find(self.inner()).type_label())
                }

                fn canonical_name(&self) -> String {
                    format!($canonical_name_pattern, DartCodeOracle::find(self.inner()).canonical_name())
                }
            }
        }
    }
 }

macro_rules! impl_renderable_for_compound {
    ($T:ty, $type_label_pattern:literal, $canonical_name_pattern: literal) => {
       paste! {
            impl Renderable for $T {
                fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                    type_helper.include_once_check($canonical_name_pattern, &self.self_type);
                    let inner_codetype = DartCodeOracle::find(self.inner());
                    let inner_type_label = inner_codetype.type_label();

                    type_helper.include_once_check(&inner_codetype.canonical_name(), &self.inner()); // Add the Inner FFI Converter

                    let cl_name = &format!($canonical_name_pattern, inner_codetype.canonical_name());
                    let type_label = &format!($type_label_pattern, &inner_type_label);

                    let inner_cl_converter_name = &inner_codetype.ffi_converter_name();
                    let inner_data_type = &inner_codetype.canonical_name().replace("UInt", "Uint").replace("Double", "Float");
                    let _inner_type_signature = if inner_data_type.contains("Float") { "double" } else { "int" };


                    quote! {
                        class $cl_name {

                            static $type_label lift(Api api, RustBuffer buf) {
                                return $cl_name.read(api, buf.asUint8List()).value;
                            }

                            static LiftRetVal<$type_label> read(Api api, Uint8List buf) {
                                if (ByteData.view(buf.buffer, buf.offsetInBytes).getInt8(0) == 0){
                                    return LiftRetVal(null, 1);
                                }
                                return $inner_cl_converter_name.read(api, Uint8List.view(buf.buffer, buf.offsetInBytes + 1));
                            }

                            static RustBuffer lower(Api api, $type_label value) {
                                return $cl_name.lowerIntoRustBuffer(api, value);
                            }

                            static RustBuffer lowerIntoRustBuffer(Api api, $type_label value, [int offset = 0]) {
                                if (value == null) {
                                    return toRustBuffer(api, Uint8List.fromList([0]));
                                }
                                throw "not yet supported";
                            //     final total_len = $inner_cl_converter_name.rustLen(api, value) + offset;
                            //     // converting the inner
                            //     final inner = $inner_cl_converter_name.lowerIntoRustBuffer(api, value);
                            //     // preparing the outer
                            //     final offset = 5;
                            //     final res = Uint8List(inner.length + offset);
                            //     // first byte sets the option to as true
                            //     res.setAll(0, [1]);
                            //     // then set the inner size
                            //     final len = Uint32List(1);
                            //     len.first = inner.length;
                            //     res.setAll(1, len.buffer.asUint8List().reversed);
                            //     // then add the actual data
                            //     res.setAll(offset, inner);
                            //     return toRustBuffer(api, res);
                            }

                            // @override
                            // $type_label read(ByteBuffer buf) {
                            //     // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                            //     // considerations, after research on performance implications
                            //     throw UnimplementedError("Should probably implement read now");
                            // }

                            // @override
                            // int allocationSize([$type_label value]) {
                            //     return $inner_cl_converter_name().allocationSize() + 4;
                            // }

                            // @override
                            // void write($type_label value, ByteBuffer buf) {
                            //     throw UnimplementedError("Should probably implement writes now");
                            // }
                        }
                    }
                }
            }
       }
   };

   (SequenceCodeType, $canonical_name_pattern: literal) => {
        paste! {
            impl Renderable for SequenceCodeType {
                fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                    type_helper.include_once_check($canonical_name_pattern, &self.self_type);
                    let inner_codetype = DartCodeOracle::find(self.inner());
                    let inner_type_label = inner_codetype.type_label();

                    type_helper.include_once_check(&inner_codetype.canonical_name(), &self.inner()); // Add the Inner FFI Converter

                    let cl_name = &format!($canonical_name_pattern, inner_codetype.canonical_name());
                    let type_label = &format!("List<{}>", &inner_type_label);

                    let inner_cl_converter_name = &self.inner().as_codetype().ffi_converter_name();
                    let inner_data_type = &inner_codetype.canonical_name().replace("UInt", "Uint").replace("Double", "Float");
                    let _inner_type_signature = if inner_data_type.contains("Float") { "double" } else { "int" };
                    // TODO: Generate the proper lifter for each of the items

                    let (lift_fn, lower_fn) = if cl_name.contains("Bool") {
                        ("FfiConverterBool.lift(api, intlist[offset])".to_string(), "Uint8List.fromList([FfiConverterBool.lower(api, value[i])])".to_string())
                    } else if cl_name.contains("String") {
                        // Only pass the string data to the lifter
                        (inner_codetype.lift() + "(api, buf, offset + 5)" , self.inner().as_codetype().lower() + "(api, value[i]).toIntList()")
                    } else {
                        (inner_codetype.lift() + "(api, buf, offset)" ,  self.inner().as_codetype().lower() + "(api, value[i]).toIntList()")
                    };
                    let allocation_fn_expr = inner_cl_converter_name.to_owned() + ".allocationSize(item)";



                    quote! {
                        class $cl_name{
                            static LiftRetVal<$type_label> read(Api api, Uint8List buf) {
                                $type_label res = [];
                                final length = buf.buffer.asByteData().getInt32(0);
                                intlist = buf.sublist(4);


                                for (var i = 0; i < length; i++) {
                                    final item = $lift_fn;
                                    offset += $allocation_fn_expr;
                                    res.add(item);
                                }

                                return res;
                            }

                            static RustBuffer lower(Api api, $type_label value) {
                                return $cl_name.lowerIntoRustBuffer(api, value);
                            }

                            static RustBuffer lowerIntoRustBuffer(Api api, $type_label value) {
                                List<Uint8List> items = [createUint8ListFromInt(value.length)];

                                for (var i = 0; i < value.length; i++) {
                                    var inner_intlist = $lower_fn;
                                    items.add(inner_intlist);
                                }

                                Uint8List uint_list = Uint8List.fromList(items.expand((inner) => inner).toList());

                                return toRustBuffer(api, uint_list);
                            }

                            // @override
                            // $type_label read(ByteBuffer buf) {
                            //     // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                            //     // considerations, after research on performance implications
                            //     throw UnimplementedError("Should probably implement read now");
                            // }

                            // @override
                            // int allocationSize([$type_label? value]) {
                            //     // TODO: Change allocation size to use the first 4 bits of the list given
                            //     return ($inner_cl_converter_name().allocationSize() * value!.length) + 4;
                            // }

                            // @override
                            // void write($type_label value, ByteBuffer buf) {
                            //     throw UnimplementedError("Should probably implement writes now");
                            // }
                        }
                    }
                }
            }
        }
   }
}

impl_code_type_for_compound!(OptionalCodeType, "{}?", "Optional{}");
impl_code_type_for_compound!(SequenceCodeType, "List<{}>", "Sequence{}");

impl_renderable_for_compound!(OptionalCodeType, "{}?", "FfiConverterOptional{}");
impl_renderable_for_compound!(SequenceCodeType, "FfiConverterSequence{}");
