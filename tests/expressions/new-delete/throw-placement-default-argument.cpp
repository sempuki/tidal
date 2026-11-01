// tidal-rules: NEW-23
// tidal-expect: run
// tidal-note: The new expression passes one placement argument, and the allocation function has a second parameter with a default argument. The deallocation function with both parameters matches.
using size = decltype(sizeof 0);
int freed = 0;
struct Gadget {
  static void* operator new(size n, int, int = 0) { return ::operator new(n); }
  static void operator delete(void* p, int, int) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p, int) { freed = 2; ::operator delete(p); }
  static void operator delete(void* p) { freed = 3; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new (1) Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
