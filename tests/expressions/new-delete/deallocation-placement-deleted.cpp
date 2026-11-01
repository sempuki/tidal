// tidal-rules: NEW-25
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: deleted function
// tidal-error-edg: deleted function
// tidal-note: GCC accepts a deleted placement deallocation function that a new expression matches (Q-0221).
using size = decltype(sizeof 0);
struct Arena {};
struct Gadget {
  static void* operator new(size n, Arena*) { return ::operator new(n); }
  static void operator delete(void* p, Arena*) = delete;
  static void operator delete(void* p) { ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  Arena a;
  try { new (&a) Gadget; } catch (int) {}
  return 0;
}
