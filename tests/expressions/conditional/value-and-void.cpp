// tidal-rules: COND-13
// tidal-expect: reject
// tidal-error-clang: right operand to \? is void
// tidal-error-gcc: third operand to the conditional operator is of type .void.
// tidal-error-edg: must have arithmetic or unscoped enum type
// tidal-note: The value operand comes second and the void operand, a cast to void, comes third.
int g(bool c, int x) { return c ? x : (void)x; }
