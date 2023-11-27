import 'package:test/test.dart';
import 'dart:ffi';
import '../callbacks.dart';

class DartGetters extends ForeignGetters {
  @override
  bool getBool(bool v, bool argumentTwo) => v ^ argumentTwo;

  @override
  String getString(String v, bool arg2) {
    if (v == 'bad-argument') {
      throw Exception('bad argument');
    }
    if (v == 'unexpected-error') {
      throw Exception('something failed');
    }
    return arg2 ? '1234567890123' : v;
  }

  @override
  String? getOption(String? v, bool arg2) {
    if (v == 'bad-argument') {
      throw Exception('bad argument');
    }
    if (v == 'unexpected-error') {
      throw Exception('something failed');
    }
    return arg2 ? v?.toUpperCase() : v;
  }

  @override
  List<int> getList(Pointer<Int32> v, bool arg2) =>
      arg2 ? List<int>.from(v.asTypedList(0)) : [];

  @override
  void getNothing(String v) {
    if (v == 'bad-argument') {
      throw Exception('bad argument');
    }
    if (v == 'unexpected-error') {
      throw Exception('something failed');
    }
  }
}

class StoredDartStringifier extends StoredForeignStringifier {
  @override
  String fromSimpleType(int value) => 'kotlin: $value';

  // We don't test this, but we're checking that the arg type is included in the minimal list of types used
  // in the UDL.
  // If this doesn't compile, then look at TypeResolver.
  String fromComplexType(List<double?>? values) => 'kotlin: $values';
}


void main() {
  final api = Api.load();
  final callback = DartGetters();
  final rustGetters = RustGetters();

  // 1. Testing callback methods
  final bool flag = true;

  for (final v in [true, false]) {
    final expected = callback.getBool(v, flag);
    final observed = rustGetters.getBool(callback, v, flag);
    assert(expected == observed,
        "roundtripping through callback: $expected != $observed");
  }

  for (final v in [
    [1, 2],
    [0, 1]
  ]) {
    final expected = callback.getList(v, flag);
    final observed = rustGetters.getList(callback, v, flag);
    assert(expected == observed,
        "roundtripping through callback: $expected != $observed");
  }

  for (final v in ["Hello", "world"]) {
    final expected = callback.getString(v, flag);
    final observed = rustGetters.getString(callback, v, flag);
    assert(expected == observed,
        "roundtripping through callback: $expected != $observed");
  }

  for (final v in ["Some", null]) {
    final expected = callback.getOption(v, !flag);
    final observed = rustGetters.getOption(callback, v, !flag);
    assert(expected == observed,
        "roundtripping through callback: $expected != $observed");
  }

  // Additional tests
  assert(rustGetters.getStringOptionalCallback(callback, "TestString", false) ==
      "TestString");
  assert(rustGetters.getStringOptionalCallback(nullptr, "TestString", false) ==
      nullptr);

  // Should not throw
  rustGetters.getNothing(callback, "TestString");

  // Exception handling
  try {
    rustGetters.getString(callback, "bad-argument", true);
    throw Exception("Expected SimpleException.BadArgument");
  } on SimpleException.BadArgument {
    // Expected error
  }

  try {
    rustGetters.getString(callback, "unexpected-error", true);
    throw Exception("Expected SimpleException.UnexpectedException");
  } on SimpleException.UnexpectedException {
    // Expected error
  }

  try {
    rustGetters.getOption(callback, "bad-argument", true);
    throw Exception("Expected ComplexException.ReallyBadArgument");
  } on ComplexException.ReallyBadArgument catch (e) {
    // Expected error
    assert(e.code == 20);
  }

  try {
    rustGetters.getOption(callback, "unexpected-error", true);
    throw Exception("Expected ComplexException.UnexpectedErrorWithReason");
  } on ComplexException.UnexpectedErrorWithReason catch (e) {
    // Expected error
    assert(e.reason == Exception("something failed").toString());
  }

  try {
    rustGetters.getNothing(callback, "bad-argument");
    throw Exception("Expected SimpleException.BadArgument");
  } on SimpleException.BadArgument {
    // Expected error
  }

  try {
    rustGetters.getNothing(callback, "unexpected-error");
    throw Exception("Expected SimpleException.UnexpectedException");
  } on SimpleException.UnexpectedException {
    // Expected error
  }

  rustGetters.destroy();

  // 2. Pass the callback in as a constructor argument
  final DartStringifier = StoredDartStringifier();
  final rustStringifier = RustStringifier(DartStringifier);

  for (final v in [1, 2]) {
    final expected = DartStringifier.fromSimpleType(v);
    final observed = rustStringifier.fromSimpleType(v);
    assert(expected == observed,
        "callback is sent on construction: $expected != $observed");
  }

  rustStringifier.destroy();
}
