// tidal-rules: INIT-9
// tidal-expect: accept
struct Cell { int v = 5; };
constexpr int read() {
  Cell* c = new Cell;
  int r = c->v;
  delete c;
  return r;
}
static_assert(read() == 5);
