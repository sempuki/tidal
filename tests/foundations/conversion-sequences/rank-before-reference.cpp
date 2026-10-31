// tidal-rules: ICS-27 ICS-28
// tidal-expect: accept
// tidal-note: ICS-28 prefers long&& and rank prefers const int&. Rank comes first.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
One take(long&&);
Two take(const int&);
static_assert(same<decltype(take(1)), Two>);
