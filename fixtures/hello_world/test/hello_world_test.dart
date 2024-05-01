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

  test("record test", () {
    final world = api.newWorld("sarisa");
    final state = world.state();
    expect(state.name, "sarisa");
    expect(state.inhabitants, 0);

    world.inc_inhabitants();

    final state = world.state();
    // object has increased
    expect(state.inhabitants, 1);
  });

  test("stringed world test", () {
    var world = api.newWorldWithName("sari");
    expect(world.name(), "sari");
    expect(world.prefixedName("mister"), "mister sari");
    expect(world.prefixedName(null), null);
    world = world.setName("new name");
    expect(world.name(), "new name");
    world = world.setName(null);
    expect(world.name(), null);
  });
}
