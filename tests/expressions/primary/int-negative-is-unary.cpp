// tidal-rules: PRIM-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(-2147483648), long>);
static_assert(-1 < 0);
