import 'package:test/test.dart';
import '../duration_type_test.dart';

void main() {
  final api = Api.load();
  final duration = api.addDuration(1, 1);
  expect(api.getSeconds(duration), equals(1));
  expect(api.getNanos(duration), equals(1));
}
