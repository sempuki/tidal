// tidal-rules: NEW-21 NEW-23
// tidal-expect: run
using size = decltype(sizeof 0);
int freed = 0;
struct Tag {};
struct Base {
  static void* operator new(size n, Tag) { return ::operator new(n); }
  static void operator delete(void* p, Tag) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p) { freed = 2; ::operator delete(p); }
};
struct Gadget : Base { Gadget() { throw 0; } };
int main() {
  try { new (Tag{}) Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
