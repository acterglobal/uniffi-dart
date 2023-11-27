import 'package:test/test.dart';
import '../dispose.dart';

void main() {
  final api = Api.load();

  test('ObjectDecrementsLiveCount', () {
    final resource = api.getResource();
    expect(api.getLiveCount(), 1);
    resource.dispose();
    expect(api.getLiveCount(), 0);
  });

  test('MapDecrementsLiveCount', () {
    final journal = api.getResourceJournalMap();
    expect(api.getLiveCount(), 2);
    journal.dispose();
    expect(api.getLiveCount(), 0);
  });

  test('ListDecrementsLiveCount', () {
    final journal = api.getResourceJournalList();
    expect(api.getLiveCount(), 2);
    journal.dispose();
    expect(api.getLiveCount(), 0);
  });

  test('MapListDecrementsLiveCount', () {
    final journal = api.getResourceJournalMapList();
    expect(api.getLiveCount(), 2);
    journal.dispose();
    expect(api.getLiveCount(), 0);
  });

  test('EnumDecrementsLiveCount', () {
    final maybeJournal = api.getMaybeResourceJournal();
    expect(api.getLiveCount(), 2);
    maybeJournal.dispose();
    expect(api.getLiveCount(), 0);
  });
}
