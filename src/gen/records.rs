use super::oracle::{AsCodeType, DartCodeOracle};
use super::render::{Renderable, TypeHelperRenderer};
use super::types::generate_type;
use super::utils::{class_name, var_name};
use genco::prelude::*;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{AsType, Record};

#[derive(Debug)]
pub struct RecordCodeType {
    id: String,
    module_path: String,
}

impl RecordCodeType {
    pub fn new(id: String, module_path: String) -> Self {
        Self { id, module_path }
    }
}

impl CodeType for RecordCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.id)
    }

    fn canonical_name(&self) -> String {
        self.id.to_string()
    }

    fn literal(&self, _literal: &Literal) -> String {
        todo!("literal not implemented for RecordCodeType");
    }
}

impl Renderable for RecordCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.check(&self.id) {
            quote!()
        } else if let Some(record_) = type_helper.get_record(&self.id) {
            generate_record(record_, type_helper)
        } else {
            todo!("render_type_helper not implemented for unknown record type");
        }
    }
}

pub fn generate_record(obj: &Record, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let cls_name = &class_name(obj.name());
    let ffi_conv_name = &class_name(&obj.as_codetype().ffi_converter_name());
    for f in obj.fields() {
        type_helper.include_once_check(&f.as_codetype().canonical_name(), &f.as_type());
    }
    quote! {
        class $cls_name {
            $(for f in obj.fields() => final $(generate_type(&f.as_type())) $(var_name(f.name()));)

            $(cls_name)._($(for f in obj.fields() => this.$(var_name(f.name())), ));
        }

        class $ffi_conv_name {
            static $cls_name lift(RustBuffer buf) {
                return $ffi_conv_name.read(buf.asUint8List()).value;
            }

            static LiftRetVal<$cls_name> read(Uint8List buf) {
                int new_offset = 0;

                $(for f in obj.fields() =>
                    final $(var_name(f.name()))_lifted = $(f.as_type().as_codetype().ffi_converter_name()).read(Uint8List.view(buf.buffer, new_offset));
                    final $(var_name(f.name())) = $(var_name(f.name()))_lifted.value;
                    new_offset += $(var_name(f.name()))_lifted.bytesRead;
                )
                return LiftRetVal($(cls_name)._(
                    $(for f in obj.fields() => $(var_name(f.name())),)
                ), new_offset);
            }

            static RustBuffer lower($cls_name value) {
                final total_length = $(for f in obj.fields() => $(f.as_type().as_codetype().ffi_converter_name()).allocationSize(value.$(var_name(f.name()))) + ) 0;
                final buf = Uint8List(total_length);
                $ffi_conv_name.write(value, buf);
                return toRustBuffer(buf);
            }

            static int write($cls_name value, Uint8List buf) {
                int new_offset = buf.offsetInBytes;

                $(for f in obj.fields() =>
                new_offset += $(f.as_type().as_codetype().ffi_converter_name()).write(value.$(var_name(f.name())), Uint8List.view(buf.buffer, new_offset));
                )
                return new_offset - buf.offsetInBytes;
            }

            static int allocationSize($cls_name value) {
                return $(for f in obj.fields() => $(f.as_type().as_codetype().ffi_converter_name()).allocationSize(value.$(var_name(f.name()))) + ) 0;
            }
        }
    }
}

