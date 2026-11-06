// X23-6: storage from ::operator new(n * sizeof(T)) for an over-aligned
// implicit-lifetime T. Does a sanitizer report the access?
#include <new>
struct alignas(64) Line { int x; };
int main() {
  void* raw = ::operator new(2 * sizeof(Line) + 16);
  // Step past the start so the address is 16-aligned and not 64-aligned.
  char* base = static_cast<char*>(raw);
  while (reinterpret_cast<decltype(sizeof 0)>(base) % 64 != 16) base += 16;
  Line* p = reinterpret_cast<Line*>(base);
  p->x = 1;
  int v = p->x;
  ::operator delete(raw);
  return v == 1 ? 0 : 1;
}
