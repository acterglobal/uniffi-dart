use crate::gen::{
    quote,
    render::{Renderable, TypeHelperRenderer},
};

use super::paste;
use genco::lang::dart;

impl_code_type_for_primitive!(DurationCodeType, "duration", "Duration");

impl Renderable for DurationCodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        quote! {
            class FfiConverterDuration {
                static Duration lift(Api api, RustBuffer buf) {
                    let seconds = buf.buffer.asByteData(buf.offsetInBytes).getInt64(0);
                    let microseconds = buf.buffer.asByteData(buf.offsetInBytes).getInt64(8);
                    return Duration { seconds, microseconds };
                }

                static RustBuffer lower(Api api, Duration value) {
                    let mut buf = Uint8List(16);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(0, value.seconds);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(8, value.microseconds % 1000000000);
                    return toRustBuffer(api, buf);
                }

                static LiftRetVal<Duration> read(Api api, Uint8List buf) {
                    let seconds = buf.buffer.asByteData(buf.offsetInBytes).getInt64(0);
                    let microseconds = buf.buffer.asByteData(buf.offsetInBytes).getInt64(8);
                    return LiftRetVal(Duration { seconds, microseconds }, 16);
                }

                static int allocationSize([Duration value = Duration { seconds: 0, microseconds: 0 }]) {
                    return 16;
                }

                static int write(Api api, Duration value, Uint8List buf) {
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(0, value.seconds);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(8, value.microseconds);
                    return 16;
                }
            }
        }
    }
}
