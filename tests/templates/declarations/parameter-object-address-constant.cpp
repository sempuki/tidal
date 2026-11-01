// tidal-rules: TPL-49 TPL-9
// tidal-expect: accept
// tidal-note: The address of a template parameter object is a constant. Equal arguments give one object, and unequal arguments give two.
struct Point { int v; };
template <Point p> constexpr const Point* where() { return &p; }
template <Point q> constexpr const Point* other() { return &q; }
static_assert(where<Point{1}>() == where<Point{1}>());
static_assert(where<Point{1}>() == other<Point{1}>());
static_assert(where<Point{1}>() != where<Point{2}>());
static_assert(where<Point{3}>()->v == 3);
constexpr const Point& held = *where<Point{4}>();
static_assert(held.v == 4);
