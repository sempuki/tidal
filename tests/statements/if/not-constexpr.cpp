// tidal-rules: IF-20
// tidal-expect: reject
void f() {
  if !constexpr (true) {}
}
