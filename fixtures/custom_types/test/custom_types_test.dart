import 'package:test/test.dart';
import '../custom_types.dart';

void main() {
  final api = Api.load();
  
  final demo = api.getCustomTypesDemo(null);

  assert(demo.url.toString() == 'http://example.com/');
  
  assert(demo.handle == 123);

  demo.url = Uri.parse('http://new.example.com/');
  demo.handle = 456;
  
  assert(demo == api.getCustomTypesDemo(demo));
}
