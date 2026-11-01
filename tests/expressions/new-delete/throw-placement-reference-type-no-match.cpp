// tidal-rules: NEW-24
// tidal-expect: run
// tidal-note: The placement parameter is const Tag& and the deallocation parameter is Tag. The types differ, so no function matches.
using size = decltype(sizeof 0);
int freed = 0;
struct Tag {};
struct Gadget {
  static void* operator new(size n, const Tag&) { return ::operator new(n); }
  static void operator delete(void* p, Tag) { freed = 1; ::operator delete(p); }
  static void operator delete(void* p) { freed = 2; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { new (Tag{}) Gadget; } catch (int) {}
  return freed == 0 ? 0 : 10 + freed;
}
