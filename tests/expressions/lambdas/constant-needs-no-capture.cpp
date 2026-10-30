// tidal-rules: LAM-9
// tidal-expect: accept
int f() {
  const int limit = 4;
  constexpr int size = 2;
  auto g = [] { return limit + size; };
  return g();
}
