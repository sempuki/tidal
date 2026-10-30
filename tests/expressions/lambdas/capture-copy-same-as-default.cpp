// tidal-rules: LAM-6
// tidal-expect: reject
void f() {
  int a = 1;
  auto g = [=, a] { return a; };
}
