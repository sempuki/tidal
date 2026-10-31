// tidal-rules: TYP-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1L + 1u), long>);
static_assert(same<decltype(1LL + 1ul), unsigned long long>);
static_assert(same<decltype(1L + 1ull), unsigned long long>);
