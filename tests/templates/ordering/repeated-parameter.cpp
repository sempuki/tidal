// tidal-rules: ORD-1
// tidal-expect: accept
// tidal-note: Invented types U1 and U2 for (T, U) give T two values, so pair(T, T) is not deducible from pair(T, U) and is more specialized.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One pair(T, T);
template <class T, class U> Two pair(T, U);
static_assert(same<decltype(pair(1, 2)), One>);
static_assert(same<decltype(pair(1, 'c')), Two>);
