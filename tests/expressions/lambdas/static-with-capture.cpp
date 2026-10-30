// tidal-rules: LAM-13
// tidal-expect: reject
void f() {
  int n = 0;
  auto g = [n]() static { return n; };
}
