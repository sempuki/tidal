// tidal-rules: OVL-43 OVL-35
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*Diff.*int
// tidal-error-gcc: no match for .operator<. .operand types are .Diff. and .int.
// tidal-error-edg: no operator "<" matches these operands
// tidal-note: a < b evaluates as (a <=> b) < 0. Diff has only a three-way comparison with int. A rewritten candidate would serve Diff < 0, but the inner comparison takes none. Written directly, d < 0 for a Diff d is valid.
struct Diff { int v; };
constexpr int operator<=>(Diff d, int) { return d.v; }
struct Point {
  int v;
  constexpr Diff operator<=>(const Point& o) const { return Diff{v - o.v}; }
};
bool less(Point a, Point b) { return a < b; }
bool direct(Diff d) { return d < 0; }
