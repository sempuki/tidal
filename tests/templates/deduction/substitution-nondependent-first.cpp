// tidal-rules: DED-28
// tidal-expect: accept
// tidal-note: The second argument does not convert to Plain, so the template drops out before Hard<int> is instantiated.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
struct Plain {};
template <class T> struct Hard { using type = typename T::missing; };
template <class T> One pick(T, Plain, typename Hard<T>::type* = nullptr);
Two pick(int, int);
static_assert(same<decltype(pick(1, 2)), Two>);
