// tidal-rules: TR-12
// tidal-expect: reject
// tidal-error-clang: expected expression
// tidal-error-gcc: expected primary-expression before .;. token
// tidal-error-edg: expected an expression
// tidal-note: The binary * has no right operand, so no production derives the expression.
int twice(int value) { return value * ; }
