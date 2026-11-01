// tidal-rules: ORD-12
// tidal-expect: accept
// tidal-note: Only the first parameter takes part, and each template is at least as specialized as the other. The one without the trailing pack wins.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct One {}; struct Two {};
template <class T> One f(T);
template <class T, class... Ts> Two f(T, Ts...);
static_assert(same<decltype(f(1)), One>);
static_assert(same<decltype(f(1, 2)), Two>);
