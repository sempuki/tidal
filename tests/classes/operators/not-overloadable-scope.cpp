// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-edg: expected an operator
struct S { int operator::(); };
