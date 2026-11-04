// tidal-rules: ICS-25
// tidal-expect: accept
// tidal-note: One conversion function serves both overloads, so the second parts decide by rank.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { operator int(); };
One take(long);
Two take(int);
static_assert(same<decltype(take(Source{})), Two>);
struct Small { operator short(); };
One put(int);
Two put(long);
static_assert(same<decltype(put(Small{})), One>);
