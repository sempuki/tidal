// The curiously recurring template pattern for a static interface.
template <class D> struct Shape {
  int area() const { return static_cast<const D&>(*this).w() * static_cast<const D&>(*this).h(); }
};
struct Rect : Shape<Rect> { int w() const { return 3; } int h() const { return 4; } };
struct Square : Shape<Square> { int s = 5; int w() const { return s; } int h() const { return s; } };
template <class D> int area(const Shape<D>& s) { return s.area(); }
int main() { return area(Rect{}) + area(Square{}) == 37 ? 0 : 1; }
