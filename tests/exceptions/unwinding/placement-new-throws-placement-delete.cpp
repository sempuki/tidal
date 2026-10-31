// tidal-rules: UNW-3
// tidal-expect: run
using Size = decltype(sizeof 0);

int placed = 0;
int unplaced = 0;
struct Tag {};
struct S {
  S() { throw 0; }
  static void* operator new(Size n, Tag) { ++placed; return ::operator new(n); }
  static void operator delete(void* p, Tag) { ++unplaced; ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
};
int main() {
  try { new (Tag{}) S; }
  catch (int) {}
  return placed == 1 && unplaced == 1 ? 0 : 1;
}
