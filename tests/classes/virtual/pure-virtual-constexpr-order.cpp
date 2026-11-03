// tidal-rules: VIRT-1
// tidal-expect: accept
// tidal-note: Write report W9. constexpr and virtual can come in either order, also on a pure virtual function.
struct Shape {
  constexpr virtual int area() const = 0;
  virtual constexpr int sides() const = 0;
};
struct Square : Shape {
  constexpr int area() const override { return 4; }
  constexpr int sides() const override { return 4; }
};
constexpr int total(const Shape& s) { return s.area() + s.sides(); }
static_assert(total(Square{}) == 8);
