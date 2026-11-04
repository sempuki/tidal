// tidal-rules: CE-3 CE-30
// tidal-expect: reject
// tidal-error-gcc: .v. is not a constant expression
// tidal-error-clang: function parameter .v. with unknown value
// tidal-error-edg: value of parameter .v. .* cannot be used as a constant
constexpr int f(int v) {
  constexpr int w = v;
  return w;
}
