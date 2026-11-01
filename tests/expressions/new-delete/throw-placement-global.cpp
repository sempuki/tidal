// tidal-rules: NEW-21 NEW-23
// tidal-expect: run
using size = decltype(sizeof 0);
int freed = 0;
alignas(16) unsigned char pool[64];
void* operator new(size, int) { return pool; }
void operator delete(void*, int) noexcept { freed = 1; }
struct Gadget { Gadget() { throw 0; } };
int main() {
  try { new (5) Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
