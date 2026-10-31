// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: not pointers to compatible types
// tidal-error-gcc: invalid operands of types .int\*. and .long int\*.
// tidal-error-edg: operand types are incompatible
int a[2];
long b[2];
auto r = &a[1] - &b[0];
