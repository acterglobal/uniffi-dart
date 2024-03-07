use genco::lang::dart;
use paste::paste;
use uniffi_bindgen::backend::CodeType;
use uniffi_bindgen::interface::{Literal, Type};
use genco::prelude::*;

use crate::gen::render::{Renderable, AsRenderable, TypeHelperRenderer};
use super::oracle::{DartCodeOracle, AsCodeType};


fn render_literal(literal: &Literal, inner: &Type) -> String {
    match literal {
        Literal::Null => "null".into() ,
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

                fn literal(&self, literal: &Literal) -> String {
                    render_literal(literal, self.inner())
                }

                fn ffi_converter_name(&self) -> String {
                    format!("{}FfiConverter", self.canonical_name())
                }
            
                fn lower(&self) -> String {
                    format!("{}().lower", self.ffi_converter_name())
                }
            
                fn write(&self) -> String {
                    format!("{}().write", self.ffi_converter_name())
                }
            
                fn lift(&self) -> String {
                    format!("{}().lift", self.ffi_converter_name())
                }
            
                fn read(&self) -> String {
                    format!("{}().read", self.ffi_converter_name())
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

                    let cl_name = format!($canonical_name_pattern, inner_codetype.canonical_name()) + "FfiConverter";
                    let type_label = &format!($type_label_pattern, &inner_type_label);

                    let (lift_fn, lower_fn) = if cl_name.contains("Bool") {
                        ("BoolFfiConverter().lift(api, intlist[5])".to_string(), "Uint8List.fromList([BoolFfiConverter().lower(api, value)])".to_string())
                    } else if cl_name.contains("String") {
                        // Only pass the string data to the lifter
                        (self.inner().as_codetype().lift() + "(api, toRustBuffer(api, intlist.sublist(5)))" , self.inner().as_codetype().lower() + "(api, value).toIntList()")
                    } else {
                        (self.inner().as_codetype().lift() + "(api, buf)" ,  self.inner().as_codetype().lower() + "(api, value).toIntList()")
                    };
                    

                    let inner_cl_converter_name = self.inner().as_codetype().ffi_converter_name();
                    let inner_data_type = &inner_codetype.canonical_name().replace("UInt", "Uint").replace("Double", "Float");
                    let inner_type_signature = if inner_data_type.contains("Float") { "double" } else { "int" };


                    quote! {
                        class $cl_name extends FfiConverter<$type_label, RustBuffer> {
                            @override
                            $type_label lift(Api api, RustBuffer buf) {
                                var intlist = buf.toIntList();
                                if (intlist.isEmpty || intlist.first == 0){
                                    return null;
                                }
                                return $lift_fn;
                            }
                        
                            @override
                            RustBuffer lower(Api api, $type_label value) {
                                if (value == null) {
                                    final res = Uint8List(1);
                                    res.first = 0;
                                    return toRustBuffer(api, res);
                                }
                                // converting the inner
                                final inner = $lower_fn;
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
                                return toRustBuffer(api, res);
                            }
                        
                            @override
                            $type_label read(ByteBuffer buf) {
                                // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                                // considerations, after research on performance implications
                                throw UnimplementedError("Should probably implement read now");
                            }
                        
                            @override
                            int allocationSize([$type_label value]) {
                                return $inner_cl_converter_name().allocationSize() + 4; 
                            }
                        
                            @override
                            void write($type_label value, ByteBuffer buf) {
                                throw UnimplementedError("Should probably implement writes now");
                            }
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

                    let cl_name = format!($canonical_name_pattern, inner_codetype.canonical_name()) + "FfiConverter";
                    let type_label = &format!("List<{}>", &inner_type_label);

                    // TODO: Generate the proper lifter for each of the items

                    let (lift_fn, lower_fn) = if cl_name.contains("Bool") {
                        ("BoolFfiConverter().lift(api, intlist[offset])".to_string(), "Uint8List.fromList([BoolFfiConverter().lower(api, value[i])])".to_string())
                    } else if cl_name.contains("String") {
                        // Only pass the string data to the lifter
                        (self.inner().as_codetype().lift() + "(api, toRustBuffer(api, intlist.sublist(offset + 5)))" , self.inner().as_codetype().lower() + "(api, value[i]).toIntList()")
                    } else {
                        (self.inner().as_codetype().lift() + "(api, toRustBuffer(api, intlist.sublist(offset)))" ,  self.inner().as_codetype().lower() + "(api, value[i]).toIntList()")
                    };
                    

                    let inner_cl_converter_name = &self.inner().as_codetype().ffi_converter_name();
                    let inner_data_type = &inner_codetype.canonical_name().replace("UInt", "Uint").replace("Double", "Float");
                    let inner_type_signature = if inner_data_type.contains("Float") { "double" } else { "int" };


                    quote! {
                        class $cl_name extends FfiConverter<$type_label, RustBuffer> {
                            @override
                            $type_label lift(Api api, RustBuffer buf) {
                                $type_label res = [];
                                var intlist = buf.toIntList();
                                final length = intlist.buffer.asByteData().getInt32(0);
                                intlist = intlist.sublist(4);
                                
                                final element_byte_size = (intlist.length ~/ length);
                                var offset = 0;
                                
                                for (var i = 0; i < length; i++) {
                                    offset = element_byte_size * i; // Update the offset for the next loop
                                    final item = $lift_fn;
                                    res.add(item);
                                }
                    
                                return res;
                            }
                        
                            @override
                            RustBuffer lower(Api api, $type_label value) {
                                var element_byte_size = $inner_cl_converter_name().allocationSize();
                                int capacity = value.length * element_byte_size;
                                List<Uint8List> items = [Uint8List(capacity + 4)]; // Four bytes for the length
                                int offset = 0;

                                // Set the length of the vec
                                items[0].setAll(offset, createUint8ListFromInt(capacity));
                                offset += 4;

                                for (var i = 0; i < value.length; i++) {
                                    var inner_intlist = $lower_fn;
                                    element_byte_size = inner_intlist.length;
                                    items.add(inner_intlist);
                                    offset += element_byte_size;
                                }

                                // TODO: Now we know enough to set the capacity and length, let's do that and flatten the list
                                throw UnimplementedError("finishung");
                                // return toRustBuffer(api, items);
                            }
                        
                            @override
                            $type_label read(ByteBuffer buf) {
                                // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                                // considerations, after research on performance implications
                                throw UnimplementedError("Should probably implement read now");
                            }
                        
                            @override
                            int allocationSize([$type_label? value]) {
                                // TODO: Change allocation size to use the first 4 bits of the list given
                                return ($inner_cl_converter_name().allocationSize() * value!.length) + 4; 
                            }
                        
                            @override
                            void write($type_label value, ByteBuffer buf) {
                                throw UnimplementedError("Should probably implement writes now");
                            }
                        }                      
                    }
                }
            }
    }
   }
}

impl_code_type_for_compound!(OptionalCodeType, "{}?", "Optional{}");
impl_code_type_for_compound!(SequenceCodeType, "List<{}>", "Sequence{}");

impl_renderable_for_compound!(OptionalCodeType, "{}?", "Optional{}");
impl_renderable_for_compound!(SequenceCodeType, "Sequence{}");