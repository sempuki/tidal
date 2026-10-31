// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-edg: expected an operator
// tidal-error-gcc: expected type-specifier before .::. token
struct S { int operator::(); };
