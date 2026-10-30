// tidal-rules: IF-20
// tidal-expect: reject
constexpr int f() {
  if consteval (true) { return 1; }
  return 2;
}
