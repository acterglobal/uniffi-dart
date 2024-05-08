import 'package:test/test.dart';
import '../duration_type_test.dart';

void main() {
  final api = Api.load();
  test('addDurationSeconds', () {
    expect(api.addDurationSeconds(2, 2), 4);
  });
  test('addDurationMilliseconds', () {
    expect(api.addDurationMilliseconds(2000, 2000), 4000);
  });
  test('addDurationMicroseconds', () {
    expect(api.addDurationMicroseconds(2000000, 2000000), 4000000);
  });
  test('getBackDurationSeconds', () {
    expect(api.getBackDurationSeconds(4), 4);
  });
  test('getBackDurationMilliseconds', () {
    expect(api.getBackDurationMilliseconds(4000), 4000);
  });
  test('getBackDurationMicroseconds', () {
    expect(api.getBackDurationMicroseconds(4000000), 4000000);
  });
}
