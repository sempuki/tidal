// tidal-rules: LAM-3
// tidal-expect: reject
void f() {
  int n = 0;
  auto g = [n] { ++n; };
}
