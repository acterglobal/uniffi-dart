use crate::gen::{
    quote,
    render::{Renderable, TypeHelperRenderer},
};

use super::paste;
use genco::lang::dart;

impl_code_type_for_primitive!(DurationCodeType, "Duration", "Duration");

impl Renderable for DurationCodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        quote! {
            class FfiConverterDuration {
                static Duration lift(RustBuffer buf) {
                    return FfiConverterDuration.read(buf.asUint8List()).value;
                }

                static RustBuffer lower(Duration value) {
                    final buf = Uint8List(allocationSize(value));
                    write(value, buf);
                    return toRustBuffer(buf);
                }

                static LiftRetVal<Duration> read(Uint8List buf) {
                    final bytes = buf.buffer.asByteData(buf.offsetInBytes, 12);
                    final seconds = bytes.getUint64(0);
                    final micros = (bytes.getUint32(8) ~/ 1000);
                    return LiftRetVal(Duration(seconds: seconds, microseconds: micros), 12);
                }

                static int allocationSize([Duration value = const Duration()]) {
                    return 12;
                }

                static int write(Duration value, Uint8List buf) {
                    final bytes = buf.buffer.asByteData(buf.offsetInBytes, 12);
                    bytes.setUint64(0, value.inSeconds);
                    final ms = (value.inMicroseconds - (value.inSeconds * 1000000)) * 1000;
                    bytes.setUint32(8, ms.toInt());
                    return 12;
                }
            }
        }
    }
}

