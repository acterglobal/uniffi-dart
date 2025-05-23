import 'package:test/test.dart';
import '../callbacks.dart'; // Adjust import to your generated code and/or callback interfaces.

class DartGetters extends ForeignGetters {
  @override
  bool getBool(bool v, bool argumentTwo) => v ^ argumentTwo;

  @override
  String getString(String v, bool arg2) {
    if (v == 'BadArgument') {
      // Throw a UniFFI-generated exception type corresponding to BadArgument
      throw SimpleException.badArgument;
    }
    if (v == 'UnexpectedException') {
      // Throw a UniFFI-generated exception type corresponding to UnexpectedError
      throw SimpleException.unexpectedError;
    }
    return arg2 ? v : '1234567890123';
  }

  @override
  String? getOption(String? v, bool arg2) {
    if (v == 'BadArgument') {
      throw ReallyBadArgumentComplexException(20); // Example of a complex error
    }
    if (v == 'UnexpectedError') {
      throw UnexpectedExceptionWithReasonComplexException("something failed");
    }
    return arg2 ? v?.toUpperCase() : v;
  }

  @override
  List<int> getList(List<int> v, bool arg2) => arg2 ? v : <int>[];

  @override
  void getNothing(String v) {
    if (v == 'BadArgument') {
      throw SimpleException.badArgument;
    }
    if (v == 'UnexpectedError') {
      throw SimpleException.unexpectedError;
    }
  }
}

class StoredDartStringifier extends StoredForeignStringifier {
  @override
  String fromSimpleType(int value) => 'kotlin: $value';

  @override
  String fromComplexType(List<double?>? values) => 'kotlin: $values';
}

void main() {
  ensureInitialized();
  // Initialize all VTables
  initForeignGettersVTable();
  initStoredForeignStringifierVTable();

  final callback = DartGetters();
  final rustGetters = RustGetters();
  final rustStringifier = RustStringifier(StoredDartStringifier());

  test('roundtrip getBool through callback', () {
    final flag = true;
    for (final v in [true, false]) {
      final expected = callback.getBool(v, flag);
      final observed = rustGetters.getBool(callback, v, flag);
      expect(observed, equals(expected));
    }
  });

  // TODO: Bring back after we've fully implemented sequences
  test('roundtrip getList through callback', () {
    final flag = true;
    for (final v in [
      [1, 2],
      [0, 1]
    ]) {
      final expected = callback.getList(v, flag);
      final observed = rustGetters.getList(callback, v, flag);
      expect(observed, equals(expected));
    }
  });

  test('roundtrip getString through callback', () {
    final flag = true;
    for (final v in ["Hello", "world"]) {
      final expected = callback.getString(v, flag);
      final observed = rustGetters.getString(callback, v, flag);
      expect(observed, equals(expected));
    }
  });

  test('roundtrip getOption through callback', () {
    final flag = true;
    for (final v in ["Some"]) {
      final expected = callback.getOption(v, flag);
      final observed = rustGetters.getOption(callback, v, flag);
      expect(observed, equals(expected));
    }
  });

  test('getStringOptionalCallback works', () {
    expect(
        rustGetters.getStringOptionalCallback(callback, "1234567890123", false),
        equals("1234567890123"));
    // Passing null as the callback
    expect(rustGetters.getStringOptionalCallback(null, "1234567890123", false),
        isNull);
  });

  test('getNothing should not throw with normal argument', () {
    // Should not throw
    rustGetters.getNothing(callback, "1234567890123");
  });

  // test('getString throws SimpleException.BadArgument', () {
  //   final v = rustGetters.getString(callback, "BadArgument", true);
  //   expect(v, throwsA(isA<Exception>()));
  // });

  // test('getString throws SimpleException.UnexpectedException', () {
  //   expect(() => rustGetters.getString(callback, "UnexpectedError", false),
  //       throwsA(isA<Exception>));
  // });

  // test('getOption throws ReallyBadArgumentComplexException', () {
  //   // We expect ReallyBadArgumentComplexException with code=20
  //   expect(
  //       () => rustGetters.getOption(callback, "BadArgument", false),
  //       throwsA(predicate(
  //           (e) => e is ReallyBadArgumentComplexException && e.code == 20)));
  // });

  // test('getOption throws UnexpectedExceptionWithReasonComplexException', () {
  //   // We expect UnexpectedExceptionWithReasonComplexException with reason matching "something failed"
  //   expect(
  //       () => rustGetters.getOption(callback, "UnexpectedError", false),
  //       throwsA(predicate((e) =>
  //           e is UnexpectedExceptionWithReasonComplexException &&
  //           e.reason == Exception("something failed").toString())));
  // });

  // test('getNothing throws SimpleException.BadArgument', () {
  //   rustGetters.getNothing(callback, "BadArgument");
  //   // expect(() => rustGetters.getNothing(callback, "BadArgument"),
  //   //     throwsA(isA<SimpleException>()));
  // });

  // test('getNothing throws SimpleException.UnexpectedException', () {
  //   rustGetters.getNothing(callback, "UnexpectedError");
  //   // expect(() => rustGetters.getNothing(callback, "UnexpectedError"),
  //   //     throwsA(isA<SimpleException>()));
  // });

  // test('destroy RustGetters', () {
  //   rustGetters.dispose();
  //   // No assertions; just ensure no errors are thrown.
  // });

  // test('RustStringifier constructed with callback', () {
  //   final dartStringifier = StoredDartStringifier();
  //   final rustStringifier2 = RustStringifier(dartStringifier);
  //   for (final v in [1, 2]) {
  //     final expected = dartStringifier.fromSimpleType(v);
  //     final observed = rustStringifier2.fromSimpleType(v);
  //     expect(observed, equals(expected));
  //   }
  //   rustStringifier2.dispose();
  // });

  // // Clean up
  // tearDownAll(() {
  //   rustStringifier.dispose();
  // });
}
