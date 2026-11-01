// tidal-rules: DCMP-13 DCMP-12
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator==.
// tidal-error-edg: no operator .==. matches
// tidal-note: The != differs from the == in its return type and its cv-qualifiers. Neither counts, so the != matches, and DCMP-6 removes the reversed ==.
struct Meter {
  bool operator==(int) const;
  int operator!=(const int);
};
bool check(Meter m) { return 1 == m; }
