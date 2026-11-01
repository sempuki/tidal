// tidal-rules: ORD-16
// tidal-expect: accept
// tidal-note: The second parameter of the first template is a non-deduced position. Deduced from the second template, it gives no value and is not checked, so the second template is at least as specialized. The other direction fails, so the second template wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T> struct Id { using type = T; };
template <class T> One f(T*, typename Id<T>::type*);
template <class T> Two f(T*, T*);
int n = 0;
static_assert(same<decltype(f(&n, &n)), Two>);
