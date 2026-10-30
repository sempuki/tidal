// tidal-rules: SB-8
// tidal-expect: reject
void f() {
  int a[2];
  auto [_, _] = a;
}
