// The non-allocating placement form with an over-aligned type.
// The first call would pass the alignment; no aligned placement form exists,
// so today the expression calls operator new(size, void*).
using size_t = decltype(sizeof 0);
inline void* operator new(size_t, void* at) noexcept { return at; }
struct alignas(64) Wide { int v; };
alignas(64) unsigned char cell[sizeof(Wide)];
int main() {
  Wide* w = new (cell) Wide{5};
  return (w->v == 5 && static_cast<void*>(w) == cell) ? 0 : 1;
}
