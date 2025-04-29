import 'package:test/test.dart';
import '../error_types.dart';

void main() {
  group('ErrorTypes Tests', () {
    test('Normal catch with explicit error interface', () {
      try {
        oops();
        fail('Must have failed');
      } on ErrorInterface catch (e) {
        expect(e.toString(),
            'because uniffi told me so\n\nCaused by:\n    oops');
        expect(e.chain().length, 2);
        expect(e.link(0), 'because uniffi told me so');
      }
    });

    test('Normal catch with implicit Arc wrapping', () {
      try {
        oopsNowrap();
        fail('Must have failed');
      } on ErrorInterface catch (e) {
        expect(e.toString(),
            'because uniffi told me so\n\nCaused by:\n    oops');
        expect(e.chain().length, 2);
        expect(e.link(0), 'because uniffi told me so');
      }
    });

    test('ErrorTrait implementation', () {
      try {
        toops();
        fail('Must have failed');
      } on ErrorTrait catch (e) {
        expect(e.msg(), 'trait-oops');
      }
    });

    test('Get error instance', () {
      final e = getError('the error');
      expect(e.toString(), 'the error');
      expect(e.link(0), 'the error');
    });

    test('Throw RichError', () {
      try {
        throwRich('oh no');
        fail('Must have failed');
      } on RichError catch (e) {
        expect(e.toString(), 'RichError: "oh no"');
      }
    });

    group('Enum Error Tests', () {
      test('Oops variant', () {
        expect(() => oops_enum(0), throwsA(isA<Error>()));
        try {
          oops_enum(0);
        } catch (e) {
          expect(e.toString(), 'uniffi.error_types.Exception\$Oops: ');
        }
      });

      test('Value variant', () {
        expect(() => oops_enum(1), throwsA(isA<Error>()));
        try {
          oops_enum(1);
        } catch (e) {
          expect(e.toString(),
              'uniffi.error_types.Exception\$Value: value=value');
        }
      });

      test('IntValue variant', () {
        expect(() => oops_enum(2), throwsA(isA<Error>()));
        try {
          oops_enum(2);
        } catch (e) {
          expect(e.toString(), 'uniffi.error_types.Exception\$IntValue: value=2');
        }
      });

      test('FlatInnerError variant with CaseA', () {
        expect(() => oops_enum(3), throwsA(isA<Error.FlatInnerError>()));
        try {
          oops_enum(3);
        } catch (e) {
          expect(
              e.toString(),
              'uniffi.error_types.Exception\$FlatInnerException: error=uniffi.error_types.FlatInner\$CaseA: inner');
        }
      });

      test('FlatInnerError variant with CaseB', () {
        expect(() => oops_enum(4), throwsA(isA<Error.FlatInnerError>()));
        try {
          oops_enum(4);
        } catch (e) {
          expect(
              e.toString(),
              'uniffi.error_types.Exception\$FlatInnerException: error=uniffi.error_types.FlatInner\$CaseB: NonUniffiTypeValue: value');
        }
      });

      test('InnerError variant', () {
        expect(() => oops_enum(5), throwsA(isA<Error.InnerError>()));
        try {
          oops_enum(5);
        } catch (e) {
          expect(e.toString(),
              'uniffi.error_types.Exception\$InnerException: error=uniffi.error_types.Inner\$CaseA: v1=inner');
        }
      });
    });

    group('Tuple Error Tests', () {
      test('TupleError Oops variant', () {
        expect(() => oops_tuple(0), throwsA(isA<TupleError>()));
        try {
          oops_tuple(0);
        } catch (e) {
          expect(e.toString(), "'oops'");
          expect(e.toString(), equals("TupleError.Oops('oops')"));
        }
      });

      test('TupleError Value variant', () {
        expect(() => oops_tuple(1), throwsA(isA<TupleError>()));
        try {
          oops_tuple(1);
        } catch (e) {
          expect(e.toString(), '1');
          expect(e.toString(), equals("TupleError.Value(1)"));
        }
      });

      test('Get tuple with default', () {
        final tuple = getTuple(null);
        expect(tuple.toString(), "TupleError.Oops('oops')");
        expect(getTuple(tuple), tuple);
      }
    });

    test('Async throw error', () async {
      try {
        await aoops();
        fail('Must have failed');
      } on ErrorInterface catch (e) {
        expect(e.toString(), 'async-oops');
      }
    });
  });
}
