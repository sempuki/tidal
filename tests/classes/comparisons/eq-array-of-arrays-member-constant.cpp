// tidal-rules: DCMP-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: static assertion failed
// tidal-error-edg: constant value
// tidal-note: Write suite W19, finding W19-1. Q-0403. In constant evaluation, the defaulted == compares every element of a member that is an array of arrays. Clang does. GCC gives true for two objects that differ in the second row, and EDG does not evaluate the comparison as a constant.
struct M {
  int c[2][2];
  bool operator==(const M&) const = default;
};
constexpr M a{{{1, 2}, {3, 4}}}, b{{{1, 2}, {3, 4}}}, d{{{1, 2}, {3, 5}}};
static_assert(a == b);
static_assert(!(a == d));
