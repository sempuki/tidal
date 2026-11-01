// tidal-rules: TPL-9
// tidal-expect: run
// tidal-note: Two function templates given equal Point arguments refer to one object. A different value gives another object.
struct Point { int x, y; };
template <Point P> const Point* first() { return &P; }
template <Point P> const Point* second() { return &P; }
int main() {
  return first<Point{1, 2}>() == second<Point{1, 2}>() && first<Point{1, 2}>() != first<Point{2, 1}>() ? 0 : 1;
}
