// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-clang: must be a unary or binary operator
// tidal-error-gcc: must have either (zero|one) or (one|two) argument
// tidal-error-edg: too many parameters for this operator function
struct S {};
S operator+(S, S, S);
