import 'package:test/test.dart';
import '../custom_error.dart';

void main() {
  final api = Api.load();
  test('anyhow error', () {
    expect(api.anyhowError(), 1);
  });

  test('rust panic', () {
    expect(api.internalPanic(), 1);
  });

  test('custom error', () {
    expect(api.customError(), 1);
  });
}
