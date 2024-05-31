import 'package:test/test.dart';
import '../bytes_example.dart';

void main() {
  final api = Api.load();
  test('intializes correct large byte array', () {
    List<int> byteArray = ByteArray.makeLargeByteArray(256);
    expect(byteArray.length, 256);
    for (int i = 0; i < 256; i++) {
      expect(byteArray[i], i);
    }
  });

  test('appends byte in array correctly', () {
    List<int> byteArray = ByteArray.makeByteArray();
    byteArray = ByteArray.appendByte(byteArray, 6);
    expect(byteArray, [1, 2, 3, 4, 5, 6]);
  });

  test('removes byte from array correctly', () {
    List<int> byteArray = ByteArray.makeByteArray();
    byteArray = ByteArray.removeByte(byteArray, 2);
    expect(byteArray, [1, 2, 4, 5]);
  });

  test('clearing array returns empty', () {
    List<int> byteArray = ByteArray.clearByteArray();
    expect(byteArray, isEmpty);
  });

  // test('multiple operations on byte array', () {
  //   List<int> byteArray = ByteArray.makeLargeByteArray(10);
  //   byteArray = ByteArray.appendByte(byteArray, 10);
  //   byteArray = ByteArray.setByte(byteArray, 0, 255);
  //   byteArray = ByteArray.removeByte(byteArray, 5);
  //   expect(byteArray, [255, 1, 2, 3, 4, 6, 7, 8, 9, 10]);
  // });

  // test('handle edge cases', () {
  //   // Empty array test
  //   List<int> byteArray = ByteArray.clearByteArray();
  //   expect(() => ByteArray.getByte(byteArray, 0), throwsRangeError);

  //   // Boundary index test
  //   byteArray = ByteArray.makeByteArray();
  //   expect(() => ByteArray.getByte(byteArray, 5), throwsRangeError);
  //   byteArray = ByteArray.setByte(byteArray, 4, 255);
  //   expect(byteArray[4], 255);
  // });
}
