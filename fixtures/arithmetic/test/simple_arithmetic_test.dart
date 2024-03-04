import 'package:test/test.dart';
import '../simple_arithmetic.dart';

void main() {
  final api = Api.load();
  test('2 + 2 = 4', () {
    expect(api.add(2, 2), 4);
  });
  test('2 * 8 = 16', () {
    expect(api.multiply(2, 8), 16);
  });
  test('2 / 8 = 0', () {
    expect(api.divideChecked(2, 8), 0);
  });
  test('8 / 0 = null', () {
    expect(api.divideChecked(8, 0), null);
  });
  test('8 / 2 = 4', () {
    expect(api.divide(8, 2), 4);
  });
  test('u8', () {
    expect(api.addU8(2, 2), 4);
  });
  test('u16', () {
    expect(api.addU16(2, 2), 4);
  });
  test('u64', () {
    expect(api.addU64(2, 2), 4);
  });

  test('i8', () {
    expect(api.addI8(2, 2), 4);
  });
  test('i16', () {
    expect(api.addI16(2, 2), 4);
  });
  test('i32', () {
    expect(api.addI32(2, 2), 4);
  });
  test('i64', () {
    expect(api.addI64(2, 2), 4);
  });
  test('f32', () {
    expect(api.addF32(2.0, 2.0), 4.0);
  });
  test('f64', () {
    expect(api.addF64(2.0, 2.9), 4.9);
  });

  test('get back u8', () {
    expect(api.getBackU8(4), 4);
  });
  test('get back  u16', () {
    expect(api.getBackU16(4), 4);
  });
  test('get back u64', () {
    expect(api.getBackU64(4), 4);
  });

  test('get back  i8', () {
    expect(api.getBackI8(4), 4);
  });
  test('get back f32', () {
    expect(api.getBackF32(4.0), 4.0);
  });
  test('get back f64', () {
    expect(api.getBackF64(4.9), 4.9);
  });
}
