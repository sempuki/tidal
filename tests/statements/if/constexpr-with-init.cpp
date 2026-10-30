// tidal-rules: IF-7 IF-16
// tidal-expect: accept
void f() {
  if constexpr (constexpr int n = 2; n > 1) {}
}
