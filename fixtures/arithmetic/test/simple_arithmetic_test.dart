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
}
