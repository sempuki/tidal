// tidal-rules: LNK-1
// tidal-expect: run
// tidal-sources: class-external-linkage.tu.cpp
// tidal-note: Point and Color have external linkage, so each name refers to one class and one enumeration in both units. The functions that the other unit defines take them as parameters.
struct Point { int x; };
enum class Color { red, green };
int read(Point p);
int pick(Color c);
int main() {
  return read(Point{3}) == 3 && pick(Color::green) == 1 ? 0 : 1;
}
