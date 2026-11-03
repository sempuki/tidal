// tidal-rules: OVL-43
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*Diff.*int
// tidal-error-gcc: no match for .operator>=. .operand types are .Diff. and .int.
// tidal-error-edg: no operator ">=" matches these operands
// tidal-note: a >= b needs (a <=> b) >= 0 or 0 >= (b <=> a). Diff has only a three-way comparison with int on the left, so either inner comparison needs a rewritten candidate, and the inner comparison takes none. Written directly, d >= 0 for a Diff d is valid by a reversed candidate.
struct Diff { int v; };
constexpr int operator<=>(int, Diff d) { return -d.v; }
struct Point {
  int v;
  constexpr Diff operator<=>(const Point& o) const { return Diff{v - o.v}; }
};
bool at_least(Point a, Point b) { return a >= b; }
bool direct(Diff d) { return d >= 0; }
