// tidal-rules: MUL-4
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*double
// tidal-error-gcc: invalid operands of types .double. and .int. to binary .operator%
// tidal-error-edg: must have integral or unscoped enum type
auto r = 5.0 % 2;
