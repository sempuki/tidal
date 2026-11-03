// tidal-rules: TYP-9
// tidal-expect: accept
// tidal-note: Other operators and the conditional operator.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(true ? 1.0f : 1L), float>);
static_assert(same<decltype(2.0f * 3ull), float>);
static_assert(same<decltype(1.0L - 1.0f), long double>);
static_assert(same<decltype(1.0 / 2), double>);
