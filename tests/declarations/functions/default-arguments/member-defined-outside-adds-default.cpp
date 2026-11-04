// tidal-rules: DEFARG-2
// tidal-expect: run
struct Box {
  int size(int unit);
};
int Box::size(int unit = 4) { return unit; }   // adds a default
namespace geo { int area(int side); }
int geo::area(int side = 3) { return side * side; }
int main() { return Box{}.size() == 4 && geo::area() == 9 ? 0 : 1; }
