// tidal-rules: ACC-7
// tidal-expect: run
class Point {
  int x = 2;
  friend int get_x(const Point& p) { return p.x; }
};
int main() { return get_x(Point{}) == 2 ? 0 : 1; }
