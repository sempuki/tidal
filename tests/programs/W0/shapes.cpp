// tidal-rules: ENUM-2 ENUM-3 SW-1 SW-3 DCMP-1 DCMP-6 SB-1 SB-2 SB-4 RFOR-1 FN-3 FN-7
// tidal-expect: run
// tidal-note: Write suite W0, program shapes.cpp (reports/clean-room/java-programmer/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// shapes.cpp: shape kinds, a switch, a defaulted ==, structured bindings,
// and a range-based for over a built-in array.
// Spec rules used: ENUM-P2, ENUM-2, ENUM-3 (scoped enumeration),
// SW-1, SW-3 (switch on a scoped enumeration), DCMP-1, DCMP-6 (defaulted ==
// and the rewritten !=), SB-1, SB-2, SB-4 (structured bindings),
// RFOR-1 (range-based for), LIST rules for aggregate braces.

enum class Kind { Circle, Square, Rectangle, Triangle };

struct Shape {
  Kind kind;
  int width;
  int height;   // unused for Circle and Square

  bool operator==(const Shape&) const = default;
};

// Twice the area, rounded down, so that all results are integers.
// A circle uses 3 for pi.
constexpr int double_area(const Shape& s) {
  switch (s.kind) {
    case Kind::Circle:
      return 2 * 3 * (s.width / 2) * (s.width / 2);
    case Kind::Square:
      return 2 * s.width * s.width;
    case Kind::Rectangle:
      return 2 * s.width * s.height;
    case Kind::Triangle:
      return s.width * s.height;
  }
  // Every int is a valid Kind value (ENUM-P3), so the cases above do not
  // cover all values. Without this return, GCC warns (FN-3, FN-7).
  return -1;
}

constexpr int sides(Kind k) {
  switch (k) {
    case Kind::Circle: return 0;
    case Kind::Triangle: return 3;
    default: return 4;
  }
}

struct Bounds {
  int min;
  int max;
};

// Returns the smallest and largest doubled area in the array.
Bounds area_bounds(const Shape (&list)[4]) {
  Bounds b{double_area(list[0]), double_area(list[0])};
  for (const Shape& s : list) {
    int a = double_area(s);
    if (a < b.min) b.min = a;
    if (a > b.max) b.max = a;
  }
  return b;
}

static_assert(double_area(Shape{Kind::Square, 3, 0}) == 18);
static_assert(sides(Kind::Triangle) == 3);
static_assert(Shape{Kind::Circle, 2, 0} == Shape{Kind::Circle, 2, 0});
static_assert(Shape{Kind::Circle, 2, 0} != Shape{Kind::Square, 2, 0});

int main() {
  Shape shapes[4] = {
      {Kind::Circle, 4, 0},       // 2 * 3 * 2 * 2 = 24
      {Kind::Square, 3, 0},       // 18
      {Kind::Rectangle, 2, 5},    // 20
      {Kind::Triangle, 6, 4},     // 24
  };

  int total = 0;
  int corners = 0;
  for (Shape s : shapes) {        // each s is a copy
    total += double_area(s);
    corners += sides(s.kind);
  }
  if (total != 86) return 1;
  if (corners != 11) return 2;

  auto [smallest, largest] = area_bounds(shapes);
  if (smallest != 18 || largest != 24) return 3;

  // Bind names to the members of one shape, by reference.
  auto& [kind, w, h] = shapes[2];
  w = 3;                          // changes shapes[2].width
  if (shapes[2].width != 3 || kind != Kind::Rectangle || h != 5) return 4;

  // A copy compares equal, a changed copy does not.
  Shape copy = shapes[3];
  if (!(copy == shapes[3])) return 5;
  copy.height = 1;
  if (copy == shapes[3]) return 6;

  int circles = 0;
  for (const Shape& s : shapes)
    if (s == Shape{Kind::Circle, 4, 0}) ++circles;
  return circles == 1 ? 0 : 7;
}
