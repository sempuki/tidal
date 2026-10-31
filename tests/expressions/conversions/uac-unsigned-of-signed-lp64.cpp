// tidal-rules: CONV-19
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1LL + 1ul), unsigned long long>);
