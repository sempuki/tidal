// tidal-rules: IF-14
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
auto f() {
  if constexpr (true) return 1;
  else return 2.0;
}
static_assert(same<decltype(f()), int>);
