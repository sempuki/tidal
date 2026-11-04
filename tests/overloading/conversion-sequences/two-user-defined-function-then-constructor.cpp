// tidal-rules: ICS-9
// tidal-expect: accept
// tidal-note: Source to Target needs a conversion function to Middle and then a constructor of Target, so no sequence exists.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Middle {};
struct Source { operator Middle(); };
struct Target { Target(Middle); };
One take(Target);
Two take(...);
static_assert(same<decltype(take(Source{})), Two>);
static_assert(same<decltype(take(Middle{})), One>);
