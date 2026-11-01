// tidal-rules: NEW-21 NEW-23
// tidal-expect: run
// tidal-note: The class declares no operator delete, so the search finds the global placement form.
using size = decltype(sizeof 0);
int freed = 0;
struct Tag {};
void operator delete(void* p, Tag) noexcept { freed = 1; ::operator delete(p); }
struct Gadget {
  static void* operator new(size n, Tag) { return ::operator new(n); }
  Gadget() { throw 0; }
};
int main() {
  try { new (Tag{}) Gadget; } catch (int) {}
  return freed == 1 ? 0 : 10 + freed;
}
