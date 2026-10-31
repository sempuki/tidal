// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-gcc: .v. is not a constant expression
constexpr int f(int v) {
  constexpr int w = v;
  return w;
}
