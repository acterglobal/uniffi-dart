import 'package:test/test.dart';
import '../hello_world.dart';

void main() {
  final api = Api.load();
  test('hello world', () {
    expect(api.helloWorld(), "hello world");
  });

  test('hello mikka', () {
    expect(api.hello("mikka"), "hello mikka");
  });

  test("object test", () {
    final world = api.newWorld();
    expect(world.isThere(), true);
  });
}
