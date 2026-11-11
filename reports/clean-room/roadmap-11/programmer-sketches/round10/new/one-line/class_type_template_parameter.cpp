// A class type as a constant template parameter.
struct Point { int x, y; };
template <Point P> constexpr int manhattan() { return (P.x < 0 ? -P.x : P.x) + (P.y < 0 ? -P.y : P.y); }
template <Point P> struct At { static constexpr int x = P.x; };
int main() {
  static_assert(manhattan<Point{3, -4}>() == 7);
  return At<Point{5, 6}>::x == 5 ? 0 : 1;
}
