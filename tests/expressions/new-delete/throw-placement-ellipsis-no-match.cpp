// tidal-rules: NEW-24
// tidal-expect: run
// tidal-note: A deallocation function with an ellipsis does not match the placement form with an int parameter.
using size = decltype(sizeof 0);
int freed = 0;
struct Gadget {
  static void* operator new(size n, int) { return ::operator new(n); }
  static void operator delete(void* p, ...) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p) { freed = 2; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new (1) Gadget; } catch (int) {}
  return freed == 0 ? 0 : 10 + freed;
}
