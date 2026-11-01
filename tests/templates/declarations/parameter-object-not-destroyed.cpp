// tidal-rules: TPL-49 TPL-9
// tidal-expect: run
// tidal-note: The destructor of Point throws at run time, which ends the program through std::terminate. The program exits 0, so the run never destroys the template parameter object.
struct Point {
  int v;
  constexpr ~Point() {
    if consteval { } else { throw v; }
  }
};
template <Point p> const Point* where() { return &p; }
int main() { return where<Point{1}>()->v - 1; }
