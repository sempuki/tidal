// tidal-rules: IF-19
// tidal-expect: reject
consteval int twice(int v) { return v * 2; }
constexpr int f(int v) {
  if !consteval { return twice(v); }
  return v;
}
