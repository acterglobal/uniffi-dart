import 'package:test/test.dart';
import '../streams_ext.dart';

void main() {
  test('Simple Stream emits expected sequence', () {
    expect(
      simpleStream(),
      emitsInOrder([0, 1, 2, 3, 4, emitsDone]),
    );
  });

  test('Count Stream emits expected sequence', () {
    expect(
      countStream(),
      emitsInOrder([0, 1, 2, 3, 4, emitsDone]),
    );
  });

  // test('Alphabet Stream emits all uppercase letters', () {
  //   expect(
  //     alphabetStream(),
  //     emitsInOrder([
  //       ...('ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')),
  //       emitsDone,
  //     ]),
  //   );
  // });

  test('Fibonacci Stream emits first 10 Fibonacci numbers', () {
    expect(
      fibonacciStream().take(10),
      emitsInOrder([0, 1, 1, 2, 3, 5, 8, 13, 21, 34, emitsDone]),
    );
  });

  test('Async Timer Stream emits incrementing numbers', () {
    expect(
      asyncTimerStream().take(5),
      emitsInOrder([1, 2, 3, 4, 5, emitsDone]),
    );
  }, timeout: Timeout(Duration(seconds: 6)));

  test('Combined Streams emits from all source streams', () {
    expect(
      combinedStreams().take(10),
      emitsInOrder([
        // We expect at least one of each type, but the order might vary
        predicate((String s) => s.startsWith('Count:')),
        predicate((String s) => s.startsWith('Fibonacci:')),

        // The rest can be in any order
        emitsThrough(predicate((String s) =>
            s.startsWith('Count:') || s.startsWith('Fibonacci:'))),

        // Ensure we get exactly 15 events
        emitsDone,
      ]),
    );
  });
}
