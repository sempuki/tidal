// tidal-rules: CE-3
// tidal-expect: reject
constexpr int f(int v) {
  constexpr int w = v;
  return w;
}
