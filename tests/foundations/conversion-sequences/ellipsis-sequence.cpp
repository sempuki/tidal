// tidal-rules: ICS-6
// tidal-expect: accept
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Tag {};
One any(...);
static_assert(same<decltype(any(1, 2.0, Tag{})), One>);
