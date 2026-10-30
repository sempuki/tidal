// tidal-rules: IF-14
// tidal-expect: run
template <bool B>
auto pick() {
  if constexpr (B) return 1;
  else return 2.0;
}
static_assert(__is_same(decltype(pick<true>()), int));
static_assert(__is_same(decltype(pick<false>()), double));
int main() { return 0; }
