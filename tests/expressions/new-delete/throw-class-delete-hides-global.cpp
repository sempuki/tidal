// tidal-rules: NEW-21 NEW-24
// tidal-expect: run
// tidal-note: The class declares only a placement operator delete. The search stops in the class, so neither global usual form is called.
using size = decltype(sizeof 0);
int freed = 0;
struct Tag {};
void operator delete(void* p) noexcept { freed = 1; static_cast<void>(p); }
void operator delete(void* p, size) noexcept { freed = 2; static_cast<void>(p); }
alignas(16) unsigned char pool[64];
struct Gadget {
  static void* operator new(size) { return pool; }
  static void operator delete(void*, Tag) { freed = 3; }
  Gadget() { throw 0; }
};
int main() {
  try { new Gadget; } catch (int) {}
  return freed == 0 ? 0 : 10 + freed;
}
