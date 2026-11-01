// tidal-rules: DED-25
// tidal-expect: accept
// tidal-note: The return type comes first in the function type. Substitution fails at T::type there and stops, so Hard<int> in the parameter is never instantiated.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Two {};
template <class T> struct Hard { static_assert(sizeof(T) == 0, "instantiated"); using type = int; };
template <class T> typename T::type pick(T, typename Hard<T>::type* = nullptr);
Two pick(...);
static_assert(same<decltype(pick(1)), Two>);
