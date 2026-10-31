// tidal-rules: COND-4
// tidal-expect: reject
// tidal-error-clang: operand to \? is void
// tidal-error-gcc: operand to the conditional operator is of type .void.
// tidal-error-edg: operand types are incompatible
void f();
int g(bool c) { return c ? f() : 1; }
