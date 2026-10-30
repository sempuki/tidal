// tidal-rules: IF-16
// tidal-expect: reject
void f(bool b) {
  if constexpr (b) {}
}
