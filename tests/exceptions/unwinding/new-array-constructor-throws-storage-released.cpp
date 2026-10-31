// tidal-rules: UNW-3
// tidal-expect: run
using Size = decltype(sizeof 0);

int allocs = 0;
int frees = 0;
int built = 0;
struct S {
  S() { if (built == 2) throw 0; ++built; }
  ~S() { --built; }
  static void* operator new[](Size n) { ++allocs; return ::operator new(n); }
  static void operator delete[](void* p) { ++frees; ::operator delete(p); }
};
int main() {
  try { new S[4]; }
  catch (int) {}
  return allocs == 1 && frees == 1 && built == 0 ? 0 : 1;
}
