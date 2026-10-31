// tidal-rules: TYP-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(sizeof 0), unsigned long>);
static_assert(same<decltype(alignof(int)), unsigned long>);
