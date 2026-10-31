// tidal-rules: BIT-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int \*
// tidal-error-gcc: invalid operands of types .int\*.
// tidal-error-edg: must have integral or unscoped enum type
int x;
int* p = &x;
auto r = p | 1;
