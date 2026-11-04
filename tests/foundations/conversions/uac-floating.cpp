// tidal-rules: CONV-19
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1.0f + 2.0), double>);
static_assert(same<decltype(1.0 + 2.0L), long double>);
static_assert(same<decltype(1.0f + 2LL), float>);
static_assert(same<decltype(2u * 1.0f), float>);
