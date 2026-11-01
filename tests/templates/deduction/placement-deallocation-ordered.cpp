// tidal-rules: DED-39 ORD-9
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-error-clang: exit 10
// tidal-note: Both deallocation templates match the placement form with Arena*. Partial ordering chooses the template with T*, which sets called to 1. Clang calls no deallocation function when two templates match (Q-0219).
using size = decltype(sizeof 0);
int called = 0;
struct Arena {};
struct Widget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  template <class T> static void operator delete(void* p, T*) { called = 1; ::operator delete(p); }
  template <class T> static void operator delete(void* p, T) { called = 2; ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
  Widget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Widget; } catch (int) {}
  return called == 1 ? 0 : 10 + called;
}
