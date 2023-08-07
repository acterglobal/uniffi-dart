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
    expect(api.devideChecked(2, 8), 0);
  });
  test('8 / 0 = null', () {
    expect(api.devideChecked(8, 0), null);
  });
  test('8 / 2 = 4', () {
    expect(api.devide(8, 2), 4);
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
}
