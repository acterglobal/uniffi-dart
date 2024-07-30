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

  test('void', () async {
    final time = await measureTime(() async {
      await voidFunction();
      //expect(result, null);
    });
    // Less than or equal to time
    expect(time.inMilliseconds <= 10, true);
  });

  test('sleep', () async {
    final time = await measureTime(() async {
      await sleep(200);
    });

    expect(time.inMilliseconds > 200 && time.inMilliseconds < 300, true);
  });

  test('sequential_future', () async {
    final time = await measureTime(() async {
      final resultAlice = await sayAfter(100, 'Alice');
      final resultBob = await sayAfter(200, 'Bob');
      expect(resultAlice, 'Hello, Alice!');
      expect(resultBob, 'Hello, Bob!');
    });
    expect(time.inMilliseconds > 300 && time.inMilliseconds < 400, true);
  });

  test('concurrent_future', () async {
    final time = await measureTime(() async {
      final results = await Future.wait([
        sayAfter(100, 'Alice'),
        sayAfter(200, 'Bob'),
      ]);

      expect(results[0], 'Hello, Alice!');
      expect(results[1], 'Hello, Bob!');
    });

    expect(time.inMilliseconds >= 200 && time.inMilliseconds <= 300, true);
  });

  test('with_tokio_runtime', () async {
    final time = await measureTime(() async {
      final resultAlice = await sayAfterWithTokio(200, 'Alice');
      expect(resultAlice, 'Hello, Alice (with Tokio)!');
    });
    expect(time.inMilliseconds > 200 && time.inMilliseconds < 300, true);
  });

  test('fallible_function_and_method', () async {
    final time1 = await measureTime(() async {
      try {
        fallibleMe(false);
        expect(true, true);
      } catch (exception) {
        expect(false, true); // should never be reached
      }
    });
    expect(time1.inMilliseconds <= 100, true);

    final time2 = await measureTime(() async {
      try {
        fallibleMe(true);
        expect(false, true); // should never be reached
      } catch (exception) {
        expect(true, true);
      }
    });
    expect(time2.inMilliseconds <= 100, true);
  });

  test('record', () async {
    final time = await measureTime(() async {
      final result = await newMyRecord('foo', 42);
      expect(result.a, 'foo');
      expect(result.b, 42);
    });
    // Heads-up: Sometimes this test will fail if for whatever reason, something on the host system pauses the execution of the async funtions.
    print('record: ${time.inMilliseconds}ms');
    expect(time.inMilliseconds <= 100, true);
  });

  test('broken_sleep', () async {
    final time = await measureTime(() async {
      await brokenSleep(100, 0); // calls the waker twice immediately
      await sleep(100); // wait for possible failure

      await brokenSleep(100, 100); // calls the waker a second time after 1s
      await sleep(200); // wait for possible failure
    });
    expect(time.inMilliseconds >= 400 && time.inMilliseconds <= 600, true);
  });
}
