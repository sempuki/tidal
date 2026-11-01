// tidal-rules: COMMA-9
// tidal-expect: reject
// tidal-error-clang: expected .\).
// tidal-error-gcc: expected primary-expression before .\{
// tidal-error-edg: expected an expression
int pick() { return (1, {2}); }
