// tidal-rules: NEW-25
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: deleted function
// tidal-error-edg: deleted function
// tidal-note: GCC accepts a deleted deallocation function that a new expression matches (Q-0221).
using size = decltype(sizeof 0);
struct Gadget {
  static void* operator new(size n) { return ::operator new(n); }
  static void operator delete(void* p) = delete;
  Gadget() { throw 0; }
};
int main() {
  try { new Gadget; } catch (int) {}
  return 0;
}
