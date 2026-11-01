// tidal-rules: TPL-9
// tidal-expect: accept
// tidal-note: A constant parameter of class type is an lvalue, so its address can be taken, and two uses with equal arguments give the same object.
struct Point { int x; int y; };
template <Point P> constexpr const Point* where() { return &P; }
static_assert(where<Point{1, 2}>() == where<Point{1, 2}>());
static_assert(where<Point{1, 2}>()->y == 2);
