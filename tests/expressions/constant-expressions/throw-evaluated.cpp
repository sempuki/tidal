// tidal-rules: CE-8
// tidal-expect: reject
constexpr int f(int v) {
  if (v < 0) throw v;
  return v;
}
static_assert(f(-1) == 0);
