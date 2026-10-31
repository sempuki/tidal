// tidal-rules: UNW-3
// tidal-expect: run
using Size = decltype(sizeof 0);

int allocs = 0;
int frees = 0;
struct S {
  S() { throw 0; }
  static void* operator new(Size n) { ++allocs; return ::operator new(n); }
  static void operator delete(void* p) { ++frees; ::operator delete(p); }
};
int main() {
  try { new S; }
  catch (int) {}
  return allocs == 1 && frees == 1 ? 0 : 1;
}
