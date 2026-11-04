// tidal-rules: LNK-1 LNK-5
// tidal-expect: run
// tidal-sources: class-unnamed-namespace-internal.tu.cpp
// tidal-note: Each unit has its own class Point in an unnamed namespace, with internal linkage. So each unit's read(Point) is a different function, and the two definitions do not collide.
namespace {
struct Point { int x; };
}
int read(Point p) { return p.x; }
int read_elsewhere();
int main() {
  return read(Point{1}) == 1 && read_elsewhere() == 20 ? 0 : 1;
}
