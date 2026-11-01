// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-clang: outside its lifetime
// tidal-error-gcc: accessing uninitialized member
// tidal-error-edg: access to uninitialized object
struct Point {
  int x;
  constexpr Point() : x(5) {}
};
constexpr int read_x(const Point& p) { return p.x; }
struct Holder {
  int first;
  Point point;
  constexpr Holder() : first(read_x(point)), point() {}
};
constinit Holder holder;
int main() {}
