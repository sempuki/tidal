// tidal-rules: ICS-3
// tidal-expect: accept
// tidal-note: int to long comes before the constructor.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Wide { Wide(long); };
One take(Wide);
Two take(...);
static_assert(same<decltype(take(1)), One>);
