// tidal-rules: IF-14
// tidal-expect: accept
auto f() {
  if constexpr (true) return 1;
  else return 2.0;
}
static_assert(__is_same(decltype(f()), int));
