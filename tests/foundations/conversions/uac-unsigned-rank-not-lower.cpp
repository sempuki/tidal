// tidal-rules: CONV-19
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(-1 + 0u), unsigned>);
static_assert(-1 + 0u == 4294967295u);
static_assert(same<decltype(1L + 1ul), unsigned long>);
static_assert(same<decltype(1 + 1ull), unsigned long long>);
