// tidal-rules: CE-1
// tidal-expect: accept
constexpr int f(int v) {
  try { return v + 1; } catch (...) { return -1; }
}
static_assert(f(1) == 2);
