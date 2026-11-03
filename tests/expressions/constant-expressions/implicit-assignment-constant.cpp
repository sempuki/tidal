// tidal-rules: CE-2
// tidal-expect: accept
// tidal-note: The implicit copy and move assignment operators count as constexpr.
struct Point { int x, y; };
constexpr int swap_sum() {
  Point a{1, 2}, b{3, 4};
  a = b;
  b = static_cast<Point&&>(a);
  return a.x + b.y;
}
static_assert(swap_sum() == 7);
