// tidal-rules: LAM-11
// tidal-expect: reject
void f() {
  int n = 0;
  auto g = [n] { return n; };
  decltype(g) h;
}
