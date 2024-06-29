import 'package:test/test.dart';
import '../large_enum.dart';

void main() {
  // Testing flat enums
  test('Creating/Lifting Flat Enums', () {
    // Do we get the expected enum
    expect(FlatEnum.one, newFlatOne());
    expect(FlatEnum.two, newFlatTwo());
    expect(FlatEnum.three, newFlatThree());
    expect(FlatEnum.four, newFlatFour());
  });

  test('Passing Down/Lowering Flat Enums', () {
    // Can we pass the value down to rust correctly?
    expect(takeFlatEnum(FlatEnum.one), "One");
    expect(takeFlatEnum(FlatEnum.two), "Two");
    expect(takeFlatEnum(FlatEnum.three), "Three");
    expect(takeFlatEnum(FlatEnum.four), "Four");
  });

  // Testing the complex associative types...
  final inner_value_small =
      127; // Can go beyond the max for 8bits, -127 to 127 for Int and 255 for UInt
  final inner_value = 84646234643264;
  final inner_value2 = 846;
  final inner_value_double = 643264.84646234;
  final inner_value_float = 84.68;
  final inner_bool = true;

  final inner_list = [3, 4, 4, 5, 4, 24434398, 4];

  // TODO: Collections (Maps, Vector, ...)
  U8Value u8Value = (newU8Value(inner_value_small) as U8Value);
  U16Value u16Value = (newU16Value(inner_value2) as U16Value);
  I8Value i8Value = (newI8Value(inner_value_small) as I8Value);
  I16Value i16Value = (newI16Value(inner_value2) as I16Value);

  // final mapEntry = {
  //   'u8Value': u8Value,
  //   'u16Value': u16Value,
  //   'i8Value': i8Value,
  // };

  U32Value u32Value = (newU32Value(inner_value2) as U32Value);
  U64Value u64Value = (newU64Value(inner_value) as U64Value);
  I64Value i64Value = (newI64Value(inner_value) as I64Value);
  I32Value i32Value = (newI32Value(inner_value2) as I32Value);
  F32Value f32Value = (newF32Value(inner_value_float) as F32Value);
  F64Value f64Value = (newF64Value(inner_value_double) as F64Value);

  StringValue stringValue =
      (newStringValue(inner_value.toString()) as StringValue);
  BoolValue boolValue = (newBoolValue(inner_bool) as BoolValue);

  // MapEntry newMapEntry = (newMap(mapEntry) as MapEntry);
  // PublicKeyValue publicKeyValue =
  //     (newPublicKeyValue(inner_list) as PublicKeyValue);

  PublicKeyValue publicKeyValue =
      (newPublicKeyValueWithoutArgument() as PublicKeyValue);

  test('Creating/Lifting Complex Enums', () {
    // Do we get the expected inner value? Correctly.
    expect(u8Value.value, inner_value_small);
    expect(u16Value.value, inner_value2);
    expect(i8Value.value, inner_value_small);
    expect(i16Value.value, inner_value2);

    expect(u32Value.value, inner_value2);
    expect(i64Value.value, inner_value);
    expect(u64Value.value, inner_value);
    expect(i32Value.value, inner_value2);
    // Comparing floats is a little tricky, but it can be done within a certain precision
    expect(true, (f32Value.value - inner_value_float).abs() < 1e-3);
    expect(true, (f64Value.value - inner_value_double).abs() < 1e-10);

    expect(stringValue.value, inner_value.toString());
    expect(boolValue.value, inner_bool);
    // Collections
    expect(publicKeyValue.value, inner_list);
    // expect(newMapEntry, mapEntry);
  });

  test('Passing Down/Lowering Complex Enums', () {
    // Can we pass the value down to rust correctly?
    expect(takeValue(u8Value), inner_value_small.toString());
    expect(takeValue(u16Value), inner_value2.toString());
    expect(takeValue(i8Value), inner_value_small.toString());
    expect(takeValue(i16Value), inner_value2.toString());
    expect(takeValue(u32Value), inner_value2.toString());
    expect(takeValue(i64Value), inner_value.toString());
    expect(takeValue(u64Value), inner_value.toString());
    expect(takeValue(i32Value), inner_value2.toString());
    expect(takeValue(f32Value), inner_value_float.toString());
    expect(takeValue(f64Value), inner_value_double.toString());

    expect(takeValue(stringValue), inner_value.toString());
    expect(takeValue(boolValue), inner_bool.toString());

    expect(takeValue(publicKeyValue), inner_list.toString());
  });
}
