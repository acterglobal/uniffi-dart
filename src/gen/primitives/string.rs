use crate::gen::{
    quote,
    render::{Renderable, TypeHelperRenderer},
};

use genco::lang::dart;
use uniffi_bindgen::backend::CodeType;

#[derive(Debug)]
pub struct StringCodeType;
impl CodeType for StringCodeType {
    fn type_label(&self) -> String {
        "String".to_owned()
    }
}

impl Renderable for StringCodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        quote! {
            class FfiConverterString {
                static String lift(RustBuffer buf) {
                    return utf8.decoder.convert(buf.asUint8List());
                }

                static RustBuffer lower(String value) {
                    return toRustBuffer(Utf8Encoder().convert(value));
                }

                static LiftRetVal<String> read(Uint8List buf) {
                    final end = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0) + 4;
                    return LiftRetVal(utf8.decoder.convert(buf, 4, end), end);
                }

                static int allocationSize([String value = ""]) {
                    return utf8.encoder.convert(value).length + 4;
                }

                static int write(String value, Uint8List buf) {
                    final list = utf8.encoder.convert(value);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, list.length);
                    buf.setAll(4, list);
                    return list.length + 4;
                }
            }
        }
    }
}

