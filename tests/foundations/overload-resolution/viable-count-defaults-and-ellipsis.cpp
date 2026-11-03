// tidal-rules: OVL-22
// tidal-expect: accept
// tidal-note: Each call has one candidate whose parameter count fits. The first takes one to three arguments by defaults, and the second four or more by the ellipsis.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One k(int, int = 1, int = 2);
Two k(int, int, int, int, ...);
static_assert(same<decltype(k(1)), One>);
static_assert(same<decltype(k(1, 2, 3)), One>);
static_assert(same<decltype(k(1, 2, 3, 4)), Two>);
static_assert(same<decltype(k(1, 2, 3, 4, 5, 6)), Two>);
