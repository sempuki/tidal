// tidal-rules: NEW-25
// tidal-expect: reject
// tidal-error-clang: private member
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
// tidal-note: The initialization cannot throw, and the deallocation function must still be accessible.
using size = decltype(sizeof 0);
struct Gadget {
  int value = 0;
  static void* operator new(size n) { return ::operator new(n); }
private:
  static void operator delete(void* p) { ::operator delete(p); }
};
int main() {
  Gadget* g = new Gadget;
  return g->value;
}
