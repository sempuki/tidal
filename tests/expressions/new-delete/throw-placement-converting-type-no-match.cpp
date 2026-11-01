// tidal-rules: NEW-24
// tidal-expect: run
// tidal-note: The placement parameter is int and the deallocation parameter is long. No function matches, and no usual form is called in its place.
using size = decltype(sizeof 0);
int freed = 0;
alignas(16) unsigned char pool[64];
void* operator new(size, int) { return pool; }
void operator delete(void*, long) noexcept { freed = 1; }
void operator delete(void* p) noexcept { freed = 2; static_cast<void>(p); }
void operator delete(void* p, size) noexcept { freed = 3; static_cast<void>(p); }
struct Gadget { Gadget() { throw 0; } };
int main() {
  try { new (5) Gadget; } catch (int) {}
  return freed == 0 ? 0 : 10 + freed;
}
