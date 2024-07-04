use genco::prelude::*;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{AsType, Enum};

use super::oracle::{AsCodeType, DartCodeOracle};
use super::render::{AsRenderable, Renderable, TypeHelperRenderer};

use super::utils::{class_name, enum_variant_name, var_name};

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
    let ffi_converter_name = &obj.as_codetype().ffi_converter_name();
    if obj.is_flat() {
        quote! {
            enum $cls_name {
                $(for variant in obj.variants() =>
                $(enum_variant_name(variant.name())),)
                ;
            }

            class $ffi_converter_name {
                static $cls_name lift(RustBuffer buffer) {
                    final index = buffer.asUint8List().buffer.asByteData().getInt32(0);
                    switch(index) {
                        $(for (index, variant) in obj.variants().iter().enumerate() =>
                        case $(index + 1):
                            return $cls_name.$(enum_variant_name(variant.name()));
                        )
                        default:
                            throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant");
                    }
                }

                static RustBuffer lower($cls_name input) {
                    return intToRustBuffer(input.index + 1);
                }
            }
        }
    } else {
        let mut variants = vec![];

        for (index, obj) in obj.variants().iter().enumerate() {
            for f in obj.fields() {
                type_helper.include_once_check(&f.as_codetype().canonical_name(), &f.as_type());
            }
            variants.push(quote!{
                class $(class_name(obj.name()))$cls_name extends $cls_name {
                    $(for field in obj.fields() => final $(&field.as_type().as_renderable().render_type(&field.as_type(), type_helper)) $(var_name(field.name()));  )

                    $(class_name(obj.name()))$cls_name._($(for field in obj.fields() => this.$(var_name(field.name())), ));

                    static LiftRetVal<$(class_name(obj.name()))$cls_name> read(Uint8List buf) {
                        int new_offset = buf.offsetInBytes;

                        $(for f in obj.fields() =>
                            final $(var_name(f.name()))_lifted = $(f.as_type().as_codetype().ffi_converter_name()).read(Uint8List.view(buf.buffer, new_offset));
                            final $(var_name(f.name())) = $(var_name(f.name()))_lifted.value;
                            new_offset += $(var_name(f.name()))_lifted.bytesRead;
                        )
                        return LiftRetVal($(class_name(obj.name()))$cls_name._(
                            $(for f in obj.fields() => $(var_name(f.name())),)
                        ), new_offset - buf.offsetInBytes);
                    }

                    @override
                    RustBuffer lower() {
                        final buf = Uint8List(allocationSize());
                        write(buf);
                        return toRustBuffer(buf);
                    }

                    @override
                    int allocationSize() {
                        return $(for f in obj.fields() => $(f.as_type().as_codetype().ffi_converter_name()).allocationSize($(var_name(f.name()))) + ) 4;
                    }

                    @override
                    int write(Uint8List buf) {
                        buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, $(index + 1)); // write index into first position;
                        int new_offset = buf.offsetInBytes + 4;

                        $(for f in obj.fields() =>
                        new_offset += $(f.as_type().as_codetype().ffi_converter_name()).write($(var_name(f.name())), Uint8List.view(buf.buffer, new_offset));
                        )

                        return new_offset - buf.offsetInBytes;
                    }
                }
            });
        }

        quote! {
            abstract class $cls_name {
                RustBuffer lower();
                int allocationSize();
                int write(Uint8List buf);
            }

            class $ffi_converter_name {
                static $cls_name lift(RustBuffer buffer) {
                    return $ffi_converter_name.read(buffer.asUint8List()).value;
                }

                static LiftRetVal<$cls_name> read(Uint8List buf) {
                    final index = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0);
                    final subview = Uint8List.view(buf.buffer, buf.offsetInBytes + 4);
                    switch(index) {
                        $(for (index, variant) in obj.variants().iter().enumerate() =>
                        case $(index + 1):
                            return $(variant.name())$cls_name.read(subview);
                        )
                        default:  throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant");
                    }
                }

                static RustBuffer lower($cls_name value) {
                    return value.lower();
                }

                static int allocationSize($cls_name value) {
                    return value.allocationSize();
                }

                static int write($cls_name value, Uint8List buf) {
                    return value.write(buf);
                }
            }

            $(variants)
        }
    }
}

