// tidal-rules: TARG-19
// tidal-expect: accept
// tidal-note: Class arguments are the same when each member is the same.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct Point { int x; int y; };
template <Point P> struct At {};
static_assert(same<At<Point{1, 2}>, At<Point{1, 2}>>);
static_assert(same<At<{1, 2}>, At<Point{1, 2}>>);
static_assert(!same<At<Point{1, 2}>, At<Point{2, 1}>>);
