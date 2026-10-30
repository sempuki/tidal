// tidal-rules: LAM-6
// tidal-expect: reject
void f() {
  int a = 1;
  auto g = [a, a] { return a; };
}
