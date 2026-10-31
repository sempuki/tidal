// tidal-rules: TYP-5
// tidal-expect: accept
// tidal-flags: -m32
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof(long) == sizeof(int));
static_assert(!same<long, int>);
