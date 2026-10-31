// tidal-rules: INIT-9
// tidal-expect: accept
struct Cell { int v = 5; };
constexpr int read() {
  Cell row[3];
  return row[0].v + row[2].v;
}
static_assert(read() == 10);
