// tidal-rules: TYP-9
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1.0f + 1.0), double>);
static_assert(same<decltype(1.0 + 1.0L), long double>);
static_assert(same<decltype(1.0f + 1LL), float>);
static_assert(same<decltype(1.0f + 1ull), float>);
