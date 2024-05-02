use crate::gen::{
    quote,
    render::{Renderable, TypeHelperRenderer},
};

use genco::lang::dart;

use super::paste;

impl_code_type_for_primitive!(BooleanCodeType, "bool", "Bool");

impl Renderable for BooleanCodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        quote! {
            class FfiConverterBool {

                static bool lift(Api api, int value) {
                    return value == 1;
                }

                static int lower(Api api, bool value) {
                    return value ? 1 :0;
                }

                static LiftRetVal<bool> read(Api api, Uint8List buf) {
                    return LiftRetVal(FfiConverterBool.lift(api, buf.first), 1);
                }

                static RustBuffer lowerIntoRustBuffer(Api api, bool value) {
                    return toRustBuffer(api, Uint8List.fromList([FfiConverterBool.lower(api, value)]));
                }

                // @override
                // bool read(ByteBuffer buf) {
                //     // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                //     // performance reasons
                //   throw UnimplementedError("Should probably implement read now");
                // }

                // @override
                // int allocationSize([bool value = false]) {
                //   return 1;
                // }

                // @override
                // void write(bool value, ByteBuffer buf) {
                //     throw UnimplementedError("Should probably implement read now");
                // }
            }
        }
    }
}
