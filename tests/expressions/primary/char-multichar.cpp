// tidal-rules: PRIM-6
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype('ab'), int>);
static_assert('ab' == 0x6162);
