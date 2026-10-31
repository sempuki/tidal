// tidal-rules: OVL-34
// tidal-expect: accept
// tidal-note: f(int, int) is better than each of the others. The order of the declarations does not change the result.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
Two f(long, long);
One f(int, int);
Two f(int, long);
Two f(long, int);
static_assert(same<decltype(f(1, 1)), One>);
