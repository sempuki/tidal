// tidal-rules: IF-20
// tidal-expect: reject
constexpr int f() {
  if consteval { return 1; } else return 2;
}
