// tidal-rules: PFX-6
// tidal-expect: run
// tidal-note: The parameter is a class. A plain argument uses a converting constructor, and a braced argument copy-list-initializes the parameter.
struct Point {
  int x, y;
  Point(int v) : x(v), y(v) {}
  Point(int a, int b) : x(a), y(b) {}
};
int sum(Point p) { return p.x + p.y; }
int main() { return sum(3) == 6 && sum({1, 2}) == 3 ? 0 : 1; }
