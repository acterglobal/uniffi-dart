use genco::prelude::*;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{AsType, Enum};

use super::oracle::{AsCodeType, DartCodeOracle};
use super::render::{AsRenderable, Renderable, TypeHelperRenderer};

#[derive(Debug)]
pub struct EnumCodeType {
    id: String,
}

impl EnumCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl CodeType for EnumCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.id)
    }

    fn canonical_name(&self) -> String {
        self.id.to_string()
    }

    fn literal(&self, literal: &Literal) -> String {
        if let Literal::Enum(v, _) = literal {
            format!(
                "{}{}",
                self.type_label(),
                DartCodeOracle::enum_variant_name(v)
            )
        } else {
            unreachable!();
        }
    }

    fn ffi_converter_name(&self) -> String {
        format!("FfiConverter{}", &DartCodeOracle::class_name(&self.canonical_name()))
    }
}

impl Renderable for EnumCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.check(&self.id) {
            quote!()
        } else if let Some(enum_) = type_helper.get_enum(&self.id) {
            generate_enum(enum_, type_helper)
        } else {
            unreachable!()
        }
    }
}

pub fn generate_enum(obj: &Enum, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let cls_name = &obj.as_codetype().canonical_name();
    let dart_cls_name = &DartCodeOracle::class_name(obj.name());
    let ffi_converter_name = &obj.as_codetype().ffi_converter_name();
    if obj.is_flat() {
        quote! {
            enum $dart_cls_name {
                $(for variant in obj.variants() =>
                $(DartCodeOracle::enum_variant_name(variant.name())),)
                ;
            }

            class $ffi_converter_name {
                static $dart_cls_name lift( RustBuffer buffer) {
                    final index = buffer.asUint8List().buffer.asByteData().getInt32(0);
                    switch(index) {
                        $(for (index, variant) in obj.variants().iter().enumerate() =>
                        case $(index + 1):
                            return $dart_cls_name.$(DartCodeOracle::enum_variant_name(variant.name()));
                        )
                        default:
                            throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant");
                    }
                }

                static RustBuffer lower( $dart_cls_name input) {
                    return toRustBuffer(createUint8ListFromInt(input.index + 1));
                }
            }
        }
    } else {
        let mut variants = vec![];

        for (index, variant_obj) in obj.variants().iter().enumerate() {
            for f in variant_obj.fields() {
                type_helper.include_once_check(&f.as_codetype().canonical_name(), &f.as_type());
            }
            let variant_dart_cls_name = &format!("{}{}", DartCodeOracle::class_name(variant_obj.name()), dart_cls_name);
            
            // Prepare constructor parameters
            let constructor_params = variant_obj.fields().iter().map(|field| {
                let param_name = DartCodeOracle::var_name(field.name());
                let param_type = field.as_type().as_renderable().render_type(&field.as_type(), type_helper);
                if variant_obj.fields().len() > 1 {
                    quote!(required this.$param_name)
                } else {
                    quote!(this.$param_name)
                }
            }).collect::<Vec<_>>();
            
            let constructor_param_list = if variant_obj.fields().len() > 1 {
                quote!({ $( for p in constructor_params => $p, ) })
            } else {
                quote!($( for p in constructor_params => $p, ))
            };
            
            variants.push(quote!{
                class $variant_dart_cls_name extends $dart_cls_name {
                    $(for field in variant_obj.fields() => final $(&field.as_type().as_renderable().render_type(&field.as_type(), type_helper)) $(DartCodeOracle::var_name(field.name()));  )
                    
                    // Add the public const constructor
                    $variant_dart_cls_name($constructor_param_list);

                    // Keep the private constructor used by `read`
                    $variant_dart_cls_name._($(for field in variant_obj.fields() => this.$(DartCodeOracle::var_name(field.name())), ));

                    static LiftRetVal<$variant_dart_cls_name> read( Uint8List buf) {
                        int new_offset = buf.offsetInBytes;

                        $(for f in variant_obj.fields() =>
                            final $(DartCodeOracle::var_name(f.name()))_lifted = $(f.as_type().as_codetype().ffi_converter_name()).read(Uint8List.view(buf.buffer, new_offset));
                            final $(DartCodeOracle::var_name(f.name())) = $(DartCodeOracle::var_name(f.name()))_lifted.value;
                            new_offset += $(DartCodeOracle::var_name(f.name()))_lifted.bytesRead;
                        )
                        return LiftRetVal($variant_dart_cls_name._(
                            $(for f in variant_obj.fields() => $(DartCodeOracle::var_name(f.name())),)
                        ), new_offset);
                    }

                    @override
                    RustBuffer lower() {
                        final buf = Uint8List(allocationSize());
                        write(buf);
                        return toRustBuffer(buf);
                    }

                    @override
                    int allocationSize() {
                        return $(for f in variant_obj.fields() => $(f.as_type().as_codetype().ffi_converter_name()).allocationSize($(DartCodeOracle::var_name(f.name()))) + ) 4;
                    }

                    @override
                    int write( Uint8List buf) {
                        buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, $(index + 1)); // write index into first position;
                        int new_offset = buf.offsetInBytes + 4;

                        $(for f in variant_obj.fields() =>
                        new_offset += $(f.as_type().as_codetype().ffi_converter_name()).write($(DartCodeOracle::var_name(f.name())), Uint8List.view(buf.buffer, new_offset));
                        )

                        return new_offset;
                    }
                }
            });
        }

        let implements_exception = if dart_cls_name.ends_with("Exception") {
            quote!( implements Exception)
        } else {
            quote!()
        };

        quote! {
            abstract class $dart_cls_name $implements_exception {
                RustBuffer lower();
                int allocationSize();
                int write( Uint8List buf);
            }

            class $ffi_converter_name {
                static $dart_cls_name lift( RustBuffer buffer) {
                    return $ffi_converter_name.read(buffer.asUint8List()).value;
                }

                static LiftRetVal<$dart_cls_name> read( Uint8List buf) {
                    final index = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0);
                    final subview = Uint8List.view(buf.buffer, buf.offsetInBytes + 4);
                    switch(index) {
                        $(for (index, variant) in obj.variants().iter().enumerate() =>
                        case $(index + 1):
                            return $(format!("{}{}", DartCodeOracle::class_name(variant.name()), dart_cls_name)).read(subview);
                        )
                        default:  throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant");
                    }
                }

                static RustBuffer lower( $dart_cls_name value) {
                    return value.lower();
                }

                static int allocationSize($dart_cls_name value) {
                    return value.allocationSize();
                }

                static int write( $dart_cls_name value, Uint8List buf) {
                    return value.write(buf);
                }
            }

            $(variants)
        }
    }
}
