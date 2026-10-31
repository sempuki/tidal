// tidal-rules: OVL-22
// tidal-expect: accept
// tidal-note: Each call has one candidate whose parameter count fits: by defaults, by the ellipsis, or by a pack.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Three {};
One f(int, int);
Two f(int, int, int, int = 0);
Three f(int, ...);
static_assert(same<decltype(f(1, 2)), One>);
static_assert(same<decltype(f(1, 2, 3)), Two>);
static_assert(same<decltype(f(1, 2, 3, 4, 5)), Three>);
static_assert(same<decltype(f(1)), Three>);
template <class... T> One g(T...);
Two g(int, int);
static_assert(same<decltype(g()), One>);
static_assert(same<decltype(g(1, 2, 3)), One>);
