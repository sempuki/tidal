// tidal-rules: PRIM-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1.0), double>);
static_assert(same<decltype(1.0f), float>);
static_assert(same<decltype(1.0L), long double>);
static_assert(same<decltype(1e3), double>);
static_assert(same<decltype(.5), double>);
static_assert(same<decltype(5.), double>);
