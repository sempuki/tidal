// tidal-rules: ICS-5
// tidal-expect: accept
// tidal-note: The template gives long. long to long is exact, so the template serves take. long to double is not exact, so the template does not serve scale, and the ellipsis is chosen.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { template <class T = void> operator long(); };
One take(long);
Two take(...);
static_assert(same<decltype(take(Source{})), One>);
One scale(double);
Two scale(...);
static_assert(same<decltype(scale(Source{})), Two>);
