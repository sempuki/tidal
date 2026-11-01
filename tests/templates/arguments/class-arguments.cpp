// tidal-rules: TARG-16
// tidal-expect: accept
// tidal-note: The argument initializes the parameter object, also from a braced list or a value of another type.
struct Point { int x; int y; };
struct Size { int w; constexpr Size(int v) : w(v) {} };
template <Point P> struct At { static constexpr int sum = P.x + P.y; };
template <Size S> struct Sized { static constexpr int width = S.w; };
static_assert(At<Point{1, 2}>::sum == 3);
static_assert(At<{3, 4}>::sum == 7);
static_assert(Sized<5>::width == 5);
