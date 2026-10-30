// tidal-rules: IF-16
// tidal-expect: accept
constexpr int g = 0;
void f() {
  if constexpr (&g) {}
}
