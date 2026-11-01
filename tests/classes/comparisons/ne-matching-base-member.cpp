// tidal-rules: DCMP-13 DCMP-12
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator==.
// tidal-error-edg: no operator .==. matches
// tidal-note: Member lookup in Meter finds the != of the base. The != matches the ==, so DCMP-6 removes the reversed ==.
struct Base { bool operator!=(int) const; };
struct Meter : Base { bool operator==(int) const; };
bool check(Meter m) { return 1 == m; }
