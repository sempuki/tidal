// tidal-rules: CONV-19
// tidal-expect: accept
// tidal-flags: -m32
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1L + 1u), unsigned long>);
static_assert(-1L + 1u == 0ul);
