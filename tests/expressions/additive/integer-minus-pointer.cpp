// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int.*int \*
// tidal-error-gcc: invalid operands of types .int. and .int\*. to binary .operator-
// tidal-error-edg: must have arithmetic or unscoped enum type
int a[2];
auto r = 1 - &a[0];
