// tidal-rules: OVL-25
// tidal-expect: accept
// tidal-note: The first function is better on the first argument and equal on the second, so it is better.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One f(int, long);
Two f(long, long);
static_assert(same<decltype(f(1, 1)), One>);
