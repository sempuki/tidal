// tidal-rules: THR-13
// tidal-expect: run
// tidal-note: A large exception object, thrown again by copy from a handler. The program replaces the global operator new, and no throw calls it.
using size = decltype(sizeof 0);
int calls = 0;
alignas(64) unsigned char pool[8192];
size used = 0;
void* operator new(size n) {
  ++calls;
  void* p = pool + used;
  used += (n + 63) / 64 * 64;
  return p;
}
void operator delete(void*) noexcept {}
void operator delete(void*, size) noexcept {}
struct Big { int data[256]; };
int main() {
  calls = 0;
  int seen = 0;
  try {
    try {
      Big b{};
      b.data[255] = 3;
      throw b;
    } catch (Big& e) {
      e.data[0] = 4;
      throw e;
    }
  } catch (const Big& e) {
    seen = e.data[0] + e.data[255];
  }
  if (seen != 7) return 1;
  return calls == 0 ? 0 : 2;
}
