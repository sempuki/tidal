// tidal-rules: CTAD-17
// tidal-expect: accept
// tidal-note: Direct initialization can use the guide from an explicit constructor. Copy initialization leaves it out, and the guide from the other constructor gives Pick<int*>.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Pick { explicit constexpr Pick(T*) {} constexpr Pick(T) {} };
int n = 0;
Pick a(&n);
Pick b = &n;
static_assert(same<decltype(a), Pick<int>> && same<decltype(b), Pick<int*>>);
