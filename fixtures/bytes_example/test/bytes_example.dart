import 'dart:typed_data';

import 'package:test/test.dart';
import '../bytes_example.dart';

void main() {
  final api = Api.load();
  test('intializes correct large byte array', () {
    Uint8List byteArray = api.makeLargeByteArray(256);
    expect(byteArray.length, 256);
    for (int i = 0; i < 256; i++) {
      expect(byteArray[i], i);
    }
  });

  test('appends byte in array correctly', () {
    Uint8List byteArray = api.makeByteArray();
    byteArray = api.appendByte(byteArray, 6);
    expect(byteArray, [1, 2, 3, 4, 5, 6]);
  });

  test('removes byte from array correctly', () {
    Uint8List byteArray = api.makeByteArray();
    byteArray = api.removeByte(byteArray, 2);
    expect(byteArray, [1, 2, 4, 5]);
  });

  test('clearing array returns empty', () {
    Uint8List byteArray = api.clearByteArray();
    expect(byteArray, isEmpty);
  });
}
