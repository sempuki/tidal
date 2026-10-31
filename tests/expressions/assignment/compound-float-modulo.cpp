// tidal-rules: ASN-5
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*double
// tidal-error-gcc: invalid operands of types .double. and .int. to binary .operator%
// tidal-error-edg: must have integral or unscoped enum type
void f() { double d = 1; d %= 2; }
