// tidal-rules: ICS-5
// tidal-expect: accept
// tidal-note: The template gives int, and int to long is not exact, so the template is not used for take. A function that is not a template is used.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Source { template <class T = void> operator int(); };
One take(long);
Two take(...);
static_assert(same<decltype(take(Source{})), Two>);
One keep(int);
Two keep(...);
static_assert(same<decltype(keep(Source{})), One>);
struct Plain { operator int(); };
static_assert(same<decltype(take(Plain{})), One>);
