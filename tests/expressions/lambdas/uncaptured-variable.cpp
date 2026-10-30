// tidal-rules: LAM-9
// tidal-expect: reject
int f() {
  int a = 1;
  auto g = [] { return a; };
  return g();
}
