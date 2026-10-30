// tidal-rules: INIT-3
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int x();
static_assert(same<decltype(x), int()>);
