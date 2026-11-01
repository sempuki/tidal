// tidal-rules: ORD-4
// tidal-expect: accept
// tidal-note: A pack of invented types cannot give a value to the single parameter T, so f(T) is more specialized. The number of arguments in the pack takes no part.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One f(T);
template <class... Ts> Two f(Ts...);
template <class T, class... Ts> One g(T, Ts...);
template <class... Ts> Two g(Ts...);
static_assert(same<decltype(f(1)), One>);
static_assert(same<decltype(g(1)), One>);
static_assert(same<decltype(g(1, 2, 3)), One>);
