// tidal-rules: PRIM-14
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype((1)), int>);
static_assert(same<decltype((1.0f)), float>);
