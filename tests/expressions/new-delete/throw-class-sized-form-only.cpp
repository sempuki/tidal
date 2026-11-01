// tidal-rules: NEW-21 NEW-22
// tidal-expect: run
using size = decltype(sizeof 0);
int freed = 0;
struct Gadget {
  static void* operator new(size n) { return ::operator new(n); }
  static void operator delete(void* p, size) { freed = 2; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new Gadget; } catch (int) {}
  return freed == 2 ? 0 : 10 + freed;
}
