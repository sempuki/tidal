// W15 exercise 3: a concept for shapes with an area and a name, a
// constrained function that sums areas, and overloads ordered by a more
// specific concept. Written from the Tidal specification by a clean-room
// reader.
//
// Rules used:
//   CPT-1, CPT-2, CPT-24    concepts and requires expressions are bool
//                           constants
//   CPT-28                  requires-expression parameters: no object exists
//   CPT-33                  compound requirement { E } -> C: C gets
//                           decltype((E)) as its first argument
//   CPT-17, CPT-18          SameAs<double> as a type constraint
//   CPT-20                  `const Shape auto&... shapes` constrains each
//                           element of the invented pack
//   PRIM-27, PRIM-28        the fold (0.0 + ... + shapes.area())
//   CNS-2, CNS-22, CNS-23   Polygon<T> = Shape<T> && ..., so Polygon
//                           subsumes Shape and is more constrained
//   ORD-13, OVL-41          the more constrained template wins the call
//   CPT-27                  a requires expression that depends on no
//                           template parameter must be valid, so the
//                           negative test goes through a nested
//                           requirement or a concept

template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;

template <class T, class U>
concept SameAs = same<T, U>;

template <class T>
concept Shape = requires (const T& s) {
  { s.area() } -> SameAs<double>;
  { s.name() } -> SameAs<const char*>;
};

template <class T>
concept Polygon = Shape<T> && requires (const T& s) {
  { s.sides() } -> SameAs<int>;
};

struct Square {
  double side;
  constexpr double area() const { return side * side; }
  constexpr const char* name() const { return "square"; }
  constexpr int sides() const { return 4; }
};

struct Triangle {
  double base;
  double height;
  constexpr double area() const { return base * height / 2; }
  constexpr const char* name() const { return "triangle"; }
  constexpr int sides() const { return 3; }
};

struct Circle {
  double radius;
  constexpr double area() const { return 3.0 * radius * radius; }
  constexpr const char* name() const { return "circle"; }
};

// A name and no area.
struct Label {
  constexpr const char* name() const { return "label"; }
};

// An area of the wrong type.
struct Grid {
  constexpr int area() const { return 4; }
  constexpr const char* name() const { return "grid"; }
};

constexpr double total_area(const Shape auto&... shapes) {
  return (0.0 + ... + shapes.area());
}

// Two overloads. Polygon subsumes Shape, so the second wins for a polygon.
constexpr int describe(const Shape auto&) { return 1; }
constexpr int describe(const Polygon auto&) { return 2; }

static_assert(Shape<Square> && Shape<Triangle> && Shape<Circle>);
static_assert(Polygon<Square> && !Polygon<Circle>);
static_assert(!Shape<Label>);
static_assert(!Shape<Grid>);          // int is not double

// The same checks with requires expressions in static_assert. These depend
// on no template parameter, so each requirement must itself be valid
// (CPT-27). A nested requirement on a false concept-id is valid and does
// not hold.
static_assert(requires { requires Shape<Square>; });
static_assert(!requires { requires Shape<Label>; });
static_assert(requires (const Square& s) { { s.area() } -> SameAs<double>; });

// "Can I call total_area with this type?" A concept with a parameter makes
// the call depend on the parameter, so a failure gives false.
template <class T>
concept Summable = requires (const T& t) { total_area(t); };
static_assert(Summable<Circle>);
static_assert(!Summable<Label>);

static_assert(total_area() == 0.0);
static_assert(total_area(Square{2.0}) == 4.0);
static_assert(total_area(Square{2.0}, Triangle{3.0, 4.0}, Circle{1.0}) == 13.0);

static_assert(describe(Circle{1.0}) == 1);
static_assert(describe(Square{1.0}) == 2);
static_assert(describe(Triangle{1.0, 1.0}) == 2);

int main() {
  Square s{3.0};
  Circle c{2.0};
  if (total_area(s, c) != 21.0) return 1;
  if (describe(s) != 2 || describe(c) != 1) return 2;
  return 0;
}
