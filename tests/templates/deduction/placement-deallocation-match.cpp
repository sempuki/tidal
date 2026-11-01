// tidal-rules: DED-39
// tidal-expect: run
// tidal-note: The template with const T* cannot make its second parameter Arena*, so only the template with T* matches the placement form. That template sets called to 1.
using size = decltype(sizeof 0);
int called = 0;
struct Arena {};
struct Widget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  template <class T> static void operator delete(void* p, T*) { called = 1; ::operator delete(p); }
  template <class T> static void operator delete(void* p, const T*) { called = 2; ::operator delete(p); }
  static void operator delete(void* p) { ::operator delete(p); }
  Widget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Widget; } catch (int) {}
  return called == 1 ? 0 : 10 + called;
}
