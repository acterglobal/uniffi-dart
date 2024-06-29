import 'package:test/test.dart';
import '../futures.dart';

Future<Duration> measureTime(Future<void> Function() action) async {
  final start = DateTime.now();
  await action();
  final end = DateTime.now();
  return end.difference(start);
}

void main() {
  initialize();
  ensureInitialized();

  test('greet', () async {
    final result = await greet("Somebody");
    expect(result, "Hello, Somebody");
  });

  test('always_ready', () async {
    final time = await measureTime(() async {
      final result = await alwaysReady();
      expect(result, true);
    });

    expect(time.inMilliseconds < 200, true);
  });

  // test('void', () async {
  //   final time = await measureTime(() async {
  //     await voidFunction();
  //     //expect(result, null);
  //   });
  //   // Less than or equal to time
  //   expect(time.compareTo(Duration(milliseconds: 4)) <= 0, true);
  // });

  test('sleep', () async {
    final time = await measureTime(() async {
      await sleep(200);
    });

    expect(time.inMilliseconds > 200 && time.inMilliseconds < 300, true);
  });

  // test('sequential_future', () async {
  //   final time = await measureTime(() async {
  //     final resultAlice = await api.sayAfter(Duration(milliseconds: 100), 'Alice');
  //     final resultBob = await api.sayAfter(Duration(milliseconds: 200), 'Bob');
  //     expect(resultAlice, 'Hello, Alice!');
  //     expect(resultBob, 'Hello, Bob!');
  //   });
  //   expect(time.inMilliseconds > 300 && time.inMilliseconds < 400, true);
  // });

  // test('concurrent_future', () async {
  //   final time = await measureTime(() async {
  //     final resultAlice = await api.sayAfter(Duration(milliseconds: 100), 'Alice');
  //     final resultBob = await api.sayAfter(Duration(milliseconds: 200), 'Bob');
  //     expect(resultAlice, 'Hello, Alice!');
  //     expect(resultBob, 'Hello, Bob!');
  //   });
  //   expect(time.inMilliseconds > 200 && time.inMilliseconds < 300, true);
  // });

  // test('with_tokio_runtime', () async {
  //   final time = await measureTime(() async {
  //     final resultAlice = await api.sayAfterWithTokio(Duration(milliseconds: 200), 'Alice');
  //     expect(resultAlice, 'Hello, Alice (with Tokio)!');
  //   });
  //   expect(time.inMilliseconds > 200 && time.inMilliseconds < 300, true);
  // });

  // test('fallible_function_and_method', () async {
  //   final time1 = await measureTime(() {
  //     try {
  //       api.fallibleMe(false);
  //       expect(true, true);
  //     } catch (exception) {
  //       expect(false, true); // should never be reached
  //     }
  //   });
  //   print('fallible function (with result): ${time1.inMilliseconds}ms');
  //   expect(time1.compareTo(Duration(milliseconds: 100)), -1);

  //   final time2 = await measureTime(() {
  //     try {
  //       api.fallibleMe(true);
  //       expect(false, true); // should never be reached
  //     } catch (exception) {
  //       expect(true, true);
  //     }
  //   });
  //   print('fallible function (with exception): ${time2.inMilliseconds}ms');
  //   expect(time2.compareTo(Duration(milliseconds: 100)), -1);
  // });

  // test('record', () async {
  //   final time = await measureTime(() {
  //     final result = api.newMyRecord('foo', 42);
  //     expect(result.a, 'foo');
  //     expect(result.b, 42);
  //   });
  //   print('record: ${time.inMilliseconds}ms');
  //   expect(time.compareTo(Duration(milliseconds: 100)), -1);
  // });

  // test('broken_sleep', () async {
  //   final time = await measureTime(() async {
  //     await api.brokenSleep(100, 0); // calls the waker twice immediately
  //     await api.sleep(100); // wait for possible failure

  //     await api.brokenSleep(100, 100); // calls the waker a second time after 1s
  //     await api.sleep(200); // wait for possible failure
  //   });
  //   expect(time.inMilliseconds < 400 && time.inMilliseconds > 600, true);
  // });
}
