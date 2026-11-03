// tidal-rules: VIRT-1 CE-2
// tidal-expect: reject
// tidal-error-clang: non-constexpr function .area.
// tidal-error-gcc: call to non-.constexpr. function .*Square::area
// tidal-error-edg: cannot call non-constexpr function "Square::area
// tidal-note: Write report W9. The base function is constexpr, and the final overrider that the call reaches is not.
struct Shape {
  constexpr virtual int area() const = 0;
};
struct Square : Shape {
  int side;
  constexpr Square(int s) : side(s) {}
  int area() const override { return side * side; }
};
constexpr int measure(const Shape& s) { return s.area(); }
constexpr Square sq(3);
static_assert(measure(sq) == 9);
