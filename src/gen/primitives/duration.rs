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
                    final bufList = buf.asUint8List();
                    final seconds = bufList.buffer.asByteData(bufList.offsetInBytes).getInt64(0);
                    final nanoseconds = bufList.buffer.asByteData(bufList.offsetInBytes).getInt64(8);
                    return Duration(seconds: seconds, microseconds: nanoseconds ~/ 1000);
                }

                static RustBuffer lower(Api api, Duration value) {
                    final buf = Uint8List(16);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(0, value.inSeconds);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(8, value.inMicroseconds * 1000);
                    return toRustBuffer(api, buf);
                }

                static LiftRetVal<Duration> read(Api api, Uint8List buf) {
                    final seconds = buf.buffer.asByteData(buf.offsetInBytes).getInt64(0);
                    final nanoseconds = buf.buffer.asByteData(buf.offsetInBytes).getInt64(8);
                    return LiftRetVal(Duration(seconds: seconds, microseconds: nanoseconds ~/ 1000), 16);
                }

                static int allocationSize([Duration value = const Duration()]) {
                    return 16;
                }

                static int write(Api api, Duration value, Uint8List buf) {
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(0, value.inSeconds);
                    buf.buffer.asByteData(buf.offsetInBytes).setInt64(8, value.inMicroseconds * 1000);
                    return 16;
                }
            }
        }
    }
}
