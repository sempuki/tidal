// tidal-rules: ENUM-14
// tidal-expect: reject
// tidal-error-clang: not an integral constant expression
// tidal-error-gcc: enumerator value for .A. is not an integer constant
// tidal-error-edg: must have a constant value
int n = 1;
enum E { A = n };
