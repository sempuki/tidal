// tidal-rules: LAM-9
// tidal-expect: reject
int f() {
  int a = 1;
  auto outer = [] {
    auto inner = [a] { return a; };
    return inner();
  };
  return outer();
}
