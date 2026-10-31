// tidal-rules: OPR-6
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+
// tidal-error-edg: no operator .\+. matches
struct S {};
int f(S a, S b) { return a + b ? 1 : 0; }
