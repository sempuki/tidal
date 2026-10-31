// tidal-rules: CVF-11
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+
// tidal-error-edg: no operator .\+. matches
struct S { explicit operator int() const { return 1; } };
int f() { return S{} + 1; }
