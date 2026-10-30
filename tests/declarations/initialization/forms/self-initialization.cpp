// tidal-rules: INIT-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
constexpr int f() {
  int x = x + 1;
  return x;
}
static_assert(f() == 1);
