// tidal-rules: DED-13
// tidal-expect: accept
// tidal-note: The same template as in nondeduced-member-template-arguments.cpp. Explicit arguments give U its value, and the call is valid.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {};
template <class T> struct Outer { template <class U> struct In {}; };
template <class T, class U> One take(T, typename Outer<T>::template In<U>);
static_assert(same<decltype(take<int, char>(1, Outer<int>::In<char>{})), One>);
