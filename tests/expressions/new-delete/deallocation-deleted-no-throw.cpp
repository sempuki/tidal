// tidal-rules: NEW-25
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: deleted function
// tidal-error-edg: deleted function
// tidal-note: The initialization cannot throw. Clang and EDG still reject the deleted deallocation function. GCC accepts (Q-0221).
using size = decltype(sizeof 0);
struct Gadget {
  int value = 0;
  static void* operator new(size n) { return ::operator new(n); }
  static void operator delete(void* p) = delete;
};
int main() {
  Gadget* g = new Gadget;
  return g->value;
}
