// tidal-rules: ORD-10 ORD-12
// tidal-expect: accept
// tidal-note: Both parameters become int&. The lvalue reference prefers the first template, and the trailing pack prefers the second. The first wins, so the reference tie-breaker comes first.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct One {}; struct Two {};
template <class T, class... Ts> One f(T&, Ts...);
template <class T> Two f(T&&);
int n = 0;
static_assert(same<decltype(f(n)), One>);
