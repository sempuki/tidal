// tidal-rules: IF-20
// tidal-expect: reject
// tidal-error-clang: expected .\(. after .if
// tidal-error-gcc: expected .\(. before .!
void f() {
  if !constexpr (true) {}
}
