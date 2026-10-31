// tidal-rules: SHF-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*double
// tidal-error-gcc: invalid operands of types .int. and .double.
// tidal-error-edg: must have integral or unscoped enum type
auto r = 1 << 2.0;
