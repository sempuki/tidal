// tidal-rules: NEW-21 NEW-12
// tidal-expect: run
// tidal-note: The argument of the initializer throws after the allocation. The storage is freed once.
using size = decltype(sizeof 0);
int allocations = 0;
int frees = 0;
int fail() { throw 0; }
struct Gadget {
  int value;
  Gadget(int v) : value(v) {}
  static void* operator new(size n) { ++allocations; return ::operator new(n); }
  static void operator delete(void* p) { ++frees; ::operator delete(p); }
};
int main() {
  try { new Gadget(fail()); } catch (int) {}
  return allocations == 1 && frees == 1 ? 0 : 1;
}
