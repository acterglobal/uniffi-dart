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
        return "String".to_owned();
    }
}

impl Renderable for StringCodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        // This method can be expanded to generate type helper methods if needed.
        quote! {
            class FfiConverterString {
                static String lift(Api api, RustBuffer buf) {
                    // reading the entire buffer, the len is where the string finishes
                    return utf8.decoder.convert(buf.asUint8List());
                }

                static RustBuffer lower(Api api, String value) {
                    return toRustBuffer(api, Utf8Encoder().convert(value));
                }

                static LiftRetVal<String> read(Api api, Uint8List buf) {
                    final end = buf.buffer.asByteData(buf.offsetInBytes).getInt32(0) + 4;
                    return LiftRetVal(utf8.decoder.convert(buf, 4, end), end);
                }

                static int allocationSize([String value = ""]) {
                    // FIXME: doing this twice for every string is bad
                    return utf8.encoder.convert(value).length + 4; // Four additional bytes for the length data
                }

                static int write(Api api, String value, Uint8List buf) {
                    // two memcopies feels bad :(
                    final list = utf8.encoder.convert(value);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt32(0, list.length);
                    buf.setAll(4, list);
                    return list.length + 4;
                }
            }
        }
    }
}
