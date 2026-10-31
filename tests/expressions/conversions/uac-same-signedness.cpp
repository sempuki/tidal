// tidal-rules: CONV-19
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1 + 1L), long>);
static_assert(same<decltype(1u + 1ul), unsigned long>);
static_assert(same<decltype(1L + 1LL), long long>);
