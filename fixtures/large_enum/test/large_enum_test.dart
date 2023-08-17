import 'package:test/test.dart';
import '../large_enum.dart';

void main() {
  final api = Api.load();

  test('Creating/Lifting Enums', () {
    // Do we get the expected inner value? Correctly.
    final inner_value = 84646234643264;
    final inner_value2 = 846;
    final inner_bool = true;

    U32Value u32Value = (api.newU32Value(inner_value2) as U32Value);
    U64Value u64Value = (api.newU64Value(inner_value) as U64Value);
    I64Value i64Value = (api.newI64Value(inner_value) as I64Value);
    I32Value i32Value = (api.newI32Value(inner_value2) as I32Value);

    StringValue stringValue =
        (api.newStringValue(inner_value.toString()) as StringValue);
    BoolValue boolValue = (api.newBoolValue(inner_bool) as BoolValue);

    expect(u32Value.value, inner_value2);
    expect(i64Value.value, inner_value);
    expect(u64Value.value, inner_value);
    expect(i32Value.value, inner_value2);

    expect(stringValue.value, inner_value.toString());
    expect(boolValue.value, inner_bool);
  });

  // test('Passing Down/Lowering Enums', () {
  //   Function eq = const ListEquality().equals;
  //   expect(api.helloWorld(), "hello world");
  // });
}
