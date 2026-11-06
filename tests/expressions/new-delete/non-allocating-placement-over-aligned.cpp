// tidal-rules: NEW-29 STG-5
// tidal-expect: run
// tidal-note: Propose X23 (reports/clean-room/X23/report.md), finding X23-5. new (cell) Wide for an over-aligned type passes the alignment first, finds no aligned non-allocating form, and calls operator new(size_t, void*) without the alignment (NEW-29).
using size_t = decltype(sizeof 0);
inline void* operator new(size_t, void* at) noexcept { return at; }
struct alignas(64) Wide { int v; };
alignas(64) unsigned char cell[sizeof(Wide)];
int main() {
  Wide* w = new (cell) Wide{5};
  return (w->v == 5 && static_cast<void*>(w) == cell) ? 0 : 1;
}
