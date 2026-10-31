// tidal-rules: COND-6
// tidal-expect: reject
// tidal-error-clang: omitting middle operand
// tidal-error-gcc: omitted middle operand
// tidal-error-edg: expected an expression
int f(int a, int b) { return a ?: b; }
