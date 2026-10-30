// tidal-rules: IF-18
// tidal-expect: accept
consteval int g() {
  if consteval { return 1; } else { return 2; }
}
static_assert(g() == 1);
