// tidal-rules: NEW-21
// tidal-expect: run
// tidal-note: ::new uses the global functions, so a global usual form frees the storage. Both global forms are replaced (UNW-3).
using size = decltype(sizeof 0);
int freed = 0;
void operator delete(void* p) noexcept { freed = 1; static_cast<void>(p); }
void operator delete(void* p, size) noexcept { freed = 1; static_cast<void>(p); }
struct Gadget {
  static void* operator new(size n) { return ::operator new(n); }
  static void operator delete(void* p) { freed = 2; ::operator delete(p); }
  Gadget() { throw 0; }
};
int main() {
  try { ::new Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
