// tidal-rules: DED-25
// tidal-expect: accept
// tidal-note: Substitution fails at T::type and stops, so Hard<int> is never instantiated.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> struct Hard { using type = typename T::missing; };
template <class T> One pick(T, typename T::type*, typename Hard<T>::type* = nullptr);
Two pick(...);
static_assert(same<decltype(pick(1, nullptr)), Two>);
