// tidal-rules: IF-19
// tidal-expect: accept
consteval int twice(int v) { return v * 2; }
constexpr int f(int v) {
  if !consteval { return v; } else { return twice(v); }
}
