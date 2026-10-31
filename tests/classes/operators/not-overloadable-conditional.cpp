// tidal-rules: OPR-2
// tidal-expect: reject
// tidal-error-gcc: prohibits overloading .operator \?:
// tidal-error-edg: expected an operator
struct S { int operator?:(int, int); };
