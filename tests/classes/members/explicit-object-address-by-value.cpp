// tidal-rules: CLS-17
// tidal-expect: run
// tidal-note: The explicit object parameter is taken by value, and the function has a second parameter.
struct Point {
  int x;
  int shifted(this Point self, int by) { return self.x + by; }
};
int main() {
  int (*p)(Point, int) = &Point::shifted;
  Point pt{5};
  return p(pt, 2) == 7 ? 0 : 1;
}
