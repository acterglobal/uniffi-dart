import 'dart:convert';
import "dart:typed_data";
import 'package:test/test.dart';
import '../coverall.dart';

void main() {
  final api = Api.load();
  
  test('Test some_dict()', () {
    final d = api.createSomeDict();
    expect(d.text, "text");
    expect(d.maybeText, "maybe_text");
    expect(d.someBytes, Uint8List.fromList(Utf8Encoder().convert("some_bytes")));
    expect(d.maybeSomeBytes, Uint8List.fromList(Utf8Encoder().convert("maybe_some_bytes")));
    expect(d.aBool, true);
    expect(d.maybeABool, false);
    expect(d.unsigned8, 1);
    expect(d.maybeUnsigned8, 2);
    expect(d.unsigned16, 3);
    expect(d.maybeUnsigned16, 4);
    expect(d.unsigned64, BigInt.parse("18446744073709551615"));
    expect(d.maybeUnsigned64, 0);
    expect(d.signed8, 8);
    expect(d.maybeSigned8, 0);
    expect(d.signed64, 9223372036854775807);
    expect(d.maybeSigned64, 0);

    bool almostEquals(double a, double b) => (a - b).abs() < 0.000001;
    
    expect(almostEquals(d.float32, 1.2345), true);
    expect(almostEquals(d.maybeFloat32!, 22.0 / 7.0), true);
    expect(almostEquals(d.float64, 0.0), true);
    expect(almostEquals(d.maybeFloat64!, 1.0), true);

    d.coveralls?.free();
  });

  test('Test createNoneDict()', () {
    final d = api.createNoneDict();
    expect(d.text, "text");
    expect(d.maybeText, null);
    expect(d.someBytes, Uint8List.fromList(utf8.encode("some_bytes")));
    expect(d.maybeSomeBytes, null);
    expect(d.aBool, true);
    expect(d.maybeABool, null);
    expect(d.unsigned8, 1);
    expect(d.maybeUnsigned8, null);
    expect(d.unsigned16, 3);
    expect(d.maybeUnsigned16, null);
    expect(d.unsigned64, BigInt.parse("18446744073709551615"));
    expect(d.maybeUnsigned64, null);
    expect(d.signed8, 8);
    expect(d.maybeSigned8, null);
    expect(d.signed64, 9223372036854775807);
    expect(d.maybeSigned64, null);

    bool almostEquals(double a, double b) => (a - b).abs() < 0.000001;
    
    expect(true, true);
    expect(d.maybeFloat32, null);
    expect(almostEquals(d.float64, 0.0), true);
    expect(d.maybeFloat64, null);

    d.coveralls?.free();
  });

  test('Test test_arcs()', () {
    final coveralls = new Coveralls("test_arcs");
    expect(api.getNumAlive(), 1);
    expect(coveralls.strongCount(), 2);
    expect(coveralls.getOther(), null);
    coveralls.takeOther(coveralls);
    expect(coveralls.strongCount(), 3);
    expect(api.getNumAlive(), 1);
    
    final other = coveralls.getOther();
    expect(other.getName(), "test_arcs");
    other.free();
    
    expect(coveralls.takeOtherFallible() is CoverallError, true);
    expect(coveralls.takeOtherPanic("expected panic: with an arc!") is CoverallError, true);
    
    expect(coveralls.falliblePanic("Expected panic in a fallible function!") is CoverallError, true);
    coveralls.takeOther(null);
    expect(coveralls.strongCount(), 2);
  });

  test('Test test_return_objects()', () {
    final coveralls = new Coveralls("test_return_objects");
    expect(api.getNumAlive(), 1);
    expect(coveralls.strongCount(), 2);
    
    final c2 = coveralls.cloneMe();
    expect(c2.getName(), coveralls.getName());
    expect(api.getNumAlive(), 2);
    expect(c2.strongCount(), 2);
  
    coveralls.takeOther(c2);
    expect(api.etNumAlive(), 2);
    expect(coveralls.strongCount(), 2);
    expect(c2.strongCount(), 3);
    
    expect(api.getNumAlive(), 2);
    
    coveralls.free();
    expect(api.getNumAlive(), 0);
  });

  test('Test test_simple_errors()', () {
    final coveralls = new Coveralls("test_simple_errors");
    expect(coveralls.maybeThrow(true) is CoverallError, true);
    expect(coveralls.maybeThrowInto(true) is CoverallError, true);
    expect(coveralls.panic("oops") is CoverallError, true);
    coveralls.free();
  });

  test('Test test_complex_errors()', () {
    final coveralls = new Coveralls("test_complex_errors");
    // TODO: refactor test to check specific variants
    expect(coveralls.maybeThrowComplex(0) is ComplexResult, true);
    expect(coveralls.maybeThrowComplex(1) is ComplexResult, true);
    expect(coveralls.maybeThrowComplex(2) is ComplexResult, true);
    expect(coveralls.maybeThrowComplex(3) is ComplexResult, true);
    expect(coveralls.maybeThrowComplex(4) is ComplexResult, true);
    coveralls.free();
  });

  test('Test test_interfaces_in_dicts()', () {
    final coveralls = new Coveralls("test_interfaces_in_dicts");
    coveralls.addPatch(new Patch(Color.RED));
    coveralls.addRepair(
      new Repair(when: DateTime.now(), patch: new Patch(Color.BLUE)),
    );
    expect(coveralls.getRepairs().length, 2);
    coveralls.free();
  });

  test('Test test_regressions()', () {
    final coveralls = new Coveralls("test_regressions");
    expect(coveralls.getStatus("success"), "status: success");
    coveralls.free();
  });


  test('Test DictWithDefaults', () {
    var d = new DictWithDefaults();
    expect(d.name, "default-value");
    expect(d.category, null);
    expect(d.integer, 31);
    d.free();

    d = new DictWithDefaults(name: "this", category: "that", integer: 42);
    expect(d.name, "this");
    expect(d.category, "that");
    expect(d.integer, 42);
    d.free();
  });

  test('Test test_bytes()', () {
    final coveralls = new Coveralls("test_bytes");
    final reversedBytes = coveralls.reverse(utf8.encode("123"));
    expect(utf8.decode(reversedBytes), "321");
    coveralls.free();
  });
}
