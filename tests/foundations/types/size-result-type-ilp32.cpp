// tidal-rules: TYP-1
// tidal-expect: accept
// tidal-flags: -m32
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(sizeof 0), unsigned int>);
static_assert(same<decltype(alignof(int)), unsigned int>);
