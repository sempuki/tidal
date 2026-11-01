// tidal-rules: NEW-23 DED-39
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-error-clang: exit 10
// tidal-note: A function and a function template both match the placement form with Arena*. The function is called and sets freed to 1. Clang calls no deallocation function when two match (Q-0220).
using size = decltype(sizeof 0);
int freed = 0;
struct Arena {};
struct Gadget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  static void operator delete(void* p, Arena*) { freed = 1; ::operator delete(p); }
  template <class T> static void operator delete(void* p, T*) { freed = 2; ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
