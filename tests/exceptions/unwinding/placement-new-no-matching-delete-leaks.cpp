// tidal-rules: UNW-3
// tidal-expect: run
using Size = decltype(sizeof 0);

int placed = 0;
int freed = 0;
struct Tag {};
struct S {
  S() { throw 0; }
  static void* operator new(Size n, Tag) { ++placed; return ::operator new(n); }
  static void operator delete(void* p) { ++freed; ::operator delete(p); }
};
int main() {
  try { new (Tag{}) S; }
  catch (int) {}
  return placed == 1 && freed == 0 ? 0 : 1;
}
