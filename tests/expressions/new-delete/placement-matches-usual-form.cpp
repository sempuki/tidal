// tidal-rules: NEW-25
// tidal-expect: reject
// tidal-error-clang: refers to non-placement .?operator delete
// tidal-error-gcc: selects non-placement .?operator delete
// tidal-error-edg: refers to non-placement function
// tidal-note: The placement form has a second parameter of the type of sizeof 0, so its match is the usual form with two parameters.
using size = decltype(sizeof 0);
struct Gadget {
  static void* operator new(size n, size) { return ::operator new(n); }
  static void operator delete(void* p, size) { ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new (size{4}) Gadget; } catch (int) {}
  return 0;
}
