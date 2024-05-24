import 'package:test/test.dart';
import '../duration_type_test.dart';

void main() {
  final api = Api.load();
  test('rust return value seconds check', () {
    final duration = api.makeDuration(5, 0);

    expect(duration.inSeconds, 5);
    expect(api.getSeconds(duration), 5);
    expect(api.getNanos(duration), 0);
  });

  test('seconds data check from dart', () {
    final duration = Duration(seconds: 10);
    expect(api.getSeconds(duration), 10);
    expect(api.getNanos(duration), 0);
  });

  test('check nanos/micros', () {
    final duration = api.makeDuration(0, 3000);
    expect(duration.inSeconds, 0);
    expect(duration.inMicroseconds, 3);
    expect(api.getSeconds(duration), 0);
    expect(api.getNanos(duration), 3000);
  });

  test('check large values', () {
    final duration = api.makeDuration(123456789, 3000000);
    expect(duration.inSeconds, 123456789);
    expect(duration.inMicroseconds, 123456789003000);
    expect(api.getSeconds(duration), 123456789);
    expect(api.getNanos(duration), 3000000);
  });
}
