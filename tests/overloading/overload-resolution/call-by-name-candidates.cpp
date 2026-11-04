// tidal-rules: OVL-1
// tidal-expect: accept
// tidal-note: Ordinary lookup finds ::near and the template, and argument-dependent lookup adds geo::near. Deduction fails for the template, which adds no candidate.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
namespace geo {
struct Point {};
One near(Point, int);
}
Two near(geo::Point, long);
template <class T> Two near(T*, int);
geo::Point p;
static_assert(same<decltype(near(p, 1)), One>);
static_assert(same<decltype(near(p, 1L)), Two>);
