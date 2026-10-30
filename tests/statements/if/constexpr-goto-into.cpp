// tidal-rules: IF-17
// tidal-expect: reject
void f() {
  goto inside;
  if constexpr (true) {
  inside:;
  }
}
