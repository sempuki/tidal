// tidal-rules: OPR-6
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+
// tidal-error-edg: no operator .\+. matches
struct S { S(int) {} S operator+(S) const { return *this; } };
S f() { return 1 + S(2); }
