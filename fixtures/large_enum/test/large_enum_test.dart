import 'package:test/test.dart';
import '../large_enum.dart';

void main() {
  final api = Api.load();

  // Testing flat enums
  test('Creating/Lifting Flat Enums', () {
    // Do we get the expected enum
    expect(FlatEnum.one, api.newFlatOne());
    expect(FlatEnum.two, api.newFlatTwo());
    expect(FlatEnum.three, api.newFlatThree());
    expect(FlatEnum.four, api.newFlatFour());
  });

  test('Passing Down/Lowering Flat Enums', () {
    // Can we pass the value down to rust correctly?
    expect(api.takeFlatEnum(FlatEnum.one), "One");
    expect(api.takeFlatEnum(FlatEnum.two), "Two");
    expect(api.takeFlatEnum(FlatEnum.three), "Three");
    expect(api.takeFlatEnum(FlatEnum.four), "Four");
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
  U8Value u8Value = (api.newU8Value(inner_value_small) as U8Value);
  U16Value u16Value = (api.newU16Value(inner_value2) as U16Value);
  I8Value i8Value = (api.newI8Value(inner_value_small) as I8Value);
  I16Value i16Value = (api.newI16Value(inner_value2) as I16Value);

  U32Value u32Value = (api.newU32Value(inner_value2) as U32Value);
  U64Value u64Value = (api.newU64Value(inner_value) as U64Value);
  I64Value i64Value = (api.newI64Value(inner_value) as I64Value);
  I32Value i32Value = (api.newI32Value(inner_value2) as I32Value);
  F32Value f32Value = (api.newF32Value(inner_value_float) as F32Value);
  F64Value f64Value = (api.newF64Value(inner_value_double) as F64Value);

  StringValue stringValue =
      (api.newStringValue(inner_value.toString()) as StringValue);
  BoolValue boolValue = (api.newBoolValue(inner_bool) as BoolValue);

  // PublicKeyValue publicKeyValue =
  //     (api.newPublicKeyValue(inner_list) as PublicKeyValue);

  // PublicKeyValue publicKeyValue =
  //     (api.newPublicKeyValueWithoutArgument() as PublicKeyValue);

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
    // expect(publicKeyValue.value, inner_list);
  });

  test('Passing Down/Lowering Complex Enums', () {
    // Can we pass the value down to rust correctly?
    expect(api.takeValue(u8Value), inner_value_small.toString());
    expect(api.takeValue(u16Value), inner_value2.toString());
    expect(api.takeValue(i8Value), inner_value_small.toString());
    expect(api.takeValue(i16Value), inner_value2.toString());
    expect(api.takeValue(u32Value), inner_value2.toString());
    expect(api.takeValue(i64Value), inner_value.toString());
    expect(api.takeValue(u64Value), inner_value.toString());
    expect(api.takeValue(i32Value), inner_value2.toString());
    expect(api.takeValue(f32Value), inner_value_float.toString());
    expect(api.takeValue(f64Value), inner_value_double.toString());

    expect(api.takeValue(stringValue), inner_value.toString());
    expect(api.takeValue(boolValue), inner_bool.toString());

    // expect(api.takeValue(publicKeyValue), inner_list.toString());
  });
}
