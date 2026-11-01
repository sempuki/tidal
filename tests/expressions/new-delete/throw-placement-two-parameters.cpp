// tidal-rules: NEW-23
// tidal-expect: run
using size = decltype(sizeof 0);
int freed = 0;
struct Gadget {
  static void* operator new(size n, int, char) { return ::operator new(n); }
  static void operator delete(void* p, int, char) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p, int) { freed = 2; ::operator delete(p); }
  static void operator delete(void* p) { freed = 3; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new (1, 'c') Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
