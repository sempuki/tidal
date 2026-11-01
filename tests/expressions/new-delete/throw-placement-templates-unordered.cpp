// tidal-rules: NEW-24 DED-39
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: is ambiguous
// tidal-note: Two templates match the placement form, and partial ordering prefers neither. No deallocation function is called. EDG rejects the new expression as ambiguous (Q-0222).
using size = decltype(sizeof 0);
int freed = 0;
struct Arena {};
struct Gadget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  template <class T> static void operator delete(void* p, T*) { freed = 1; ::operator delete(p); }
  template <class T, class U = int> static void operator delete(void* p, T*) { freed = 2; ::operator delete(p); }
  static void operator delete(void* p) { freed = 3; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Gadget; } catch (int) {}
  return freed == 0 ? 0 : 10 + freed;
}
