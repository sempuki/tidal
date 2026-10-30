// tidal-rules: IF-14
// tidal-expect: run
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <bool B>
auto pick() {
  if constexpr (B) return 1;
  else return 2.0;
}
static_assert(same<decltype(pick<true>()), int>);
static_assert(same<decltype(pick<false>()), double>);
int main() { return 0; }
