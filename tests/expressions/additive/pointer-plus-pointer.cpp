// tidal-rules: ADD-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int \*.*int \*
// tidal-error-gcc: invalid operands of types .int\*. and .int\*. to binary .operator\+
// tidal-error-edg: must have integral or unscoped enum type
int a[2];
auto r = &a[0] + &a[1];
