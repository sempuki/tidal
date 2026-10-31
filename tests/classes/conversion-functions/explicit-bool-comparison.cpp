// tidal-rules: CVF-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator==
// tidal-error-edg: no operator .==. matches
struct H { explicit operator bool() const { return true; } };
bool f(H a) { return a == true; }
