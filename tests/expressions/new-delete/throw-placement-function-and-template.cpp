// tidal-rules: NEW-24 DED-39
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-error-gcc: exit 11
// tidal-note: A function and a function template both match the placement form with Arena*. Two matches give no deallocation function, so freed stays 0. GCC calls the function and sets freed to 1 (Q-0220).
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
  return freed == 0 ? 0 : 10 + freed;
}
