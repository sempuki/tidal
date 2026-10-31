// tidal-rules: TYP-5
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(sizeof(long) == sizeof(long long));
static_assert(!same<long, long long> && !same<unsigned long, unsigned long long>);
