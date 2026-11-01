// tidal-rules: ORD-16
// tidal-expect: accept
// tidal-note: Id<X>::type of an invented X stays unresolved and matches neither T nor int, so the second template is not deduced from the first. The first template is deduced from the second with no check of its non-deduced position. The second template wins in both calls.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T> struct Id { using type = T; };
template <class T> One f(T, typename Id<T>::type);
template <class T> Two f(T, T);
template <class T> One g(T, typename Id<T>::type);
template <class T> Two g(T, int);
static_assert(same<decltype(f(1, 1)), Two>);
static_assert(same<decltype(g(1, 1)), Two>);
