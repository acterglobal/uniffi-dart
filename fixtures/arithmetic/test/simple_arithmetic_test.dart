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
  test('u8', () {
    expect(api.add_u8(2, 2), 4);
  });
  test('u16', () {
    expect(api.add_u16(2, 2), 4);
  });
  test('u64', () {
    expect(api.add_u64(2, 2), 4);
  });

  test('i8', () {
    expect(api.add_i8(2, 2), 4);
  });
  test('i16', () {
    expect(api.add_i16(2, 2), 4);
  });
  test('i32', () {
    expect(api.add_i32(2, 2), 4);
  });
  test('i64', () {
    expect(api.add_i64(2, 2), 4);
  });
}
