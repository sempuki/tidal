// tidal-rules: NEW-25
// tidal-expect: reject
// tidal-error-clang: private member
// tidal-error-gcc: is private within this context
// tidal-error-edg: is inaccessible
using size = decltype(sizeof 0);
struct Gadget {
  static void* operator new(size n) { return ::operator new(n); }
  Gadget() { throw 0; }
private:
  static void operator delete(void* p) { ::operator delete(p); }
};
int main() {
  try { new Gadget; } catch (int) {}
  return 0;
}
