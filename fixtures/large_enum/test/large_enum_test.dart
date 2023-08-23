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
  final inner_value = 84646234643264;
  final inner_value2 = 846;
  final inner_value_double = 643264.84646234;
  final inner_value_float = 84.68;
  final inner_bool = true;
  // TODO: Add u8, i8, u16, i16, and Collections (Maps, Vector, ...)
  U32Value u32Value = (api.newU32Value(inner_value2) as U32Value);
  U64Value u64Value = (api.newU64Value(inner_value) as U64Value);
  I64Value i64Value = (api.newI64Value(inner_value) as I64Value);
  I32Value i32Value = (api.newI32Value(inner_value2) as I32Value);
  F32Value f32Value = (api.newF32Value(inner_value_float) as F32Value);
  F64Value f64Value = (api.newF64Value(inner_value_double) as F64Value);
  // TODO: Cover Floats and Doubles
  StringValue stringValue =
      (api.newStringValue(inner_value.toString()) as StringValue);
  BoolValue boolValue = (api.newBoolValue(inner_bool) as BoolValue);

  test('Creating/Lifting Complex Enums', () {
    // Do we get the expected inner value? Correctly.
    expect(u32Value.value, inner_value2);
    expect(i64Value.value, inner_value);
    expect(u64Value.value, inner_value);
    expect(i32Value.value, inner_value2);
    // Comparing floats is a little tricky, but it can be done within a certain precision
    expect(true, (f32Value.value - inner_value_float).abs() < 1e-3);
    expect(true, (f64Value.value - inner_value_double).abs() < 1e-10);

    expect(stringValue.value, inner_value.toString());
    expect(boolValue.value, inner_bool);
  });

  test('Passing Down/Lowering Complex Enums', () {
    // Can we pass the value down to rust correctly?
    expect(api.takeValue(u32Value), inner_value2.toString());
    expect(api.takeValue(i64Value), inner_value.toString());
    expect(api.takeValue(u64Value), inner_value.toString());
    expect(api.takeValue(i32Value), inner_value2.toString());
    expect(api.takeValue(f32Value), inner_value_float.toString());
    expect(api.takeValue(f64Value), inner_value_double.toString());

    expect(api.takeValue(stringValue), inner_value.toString());
    expect(api.takeValue(boolValue), inner_bool.toString());
  });
}
