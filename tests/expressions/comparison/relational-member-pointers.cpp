// tidal-rules: CMP-9
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int S::\*
// tidal-error-gcc: invalid operands of types .int S::\*. and .int S::\*.
// tidal-error-edg: must have arithmetic, unscoped enum, or pointer type
// tidal-note: Gap review 2026-10-31 (third).
struct S { int a, b; };
bool f() { return &S::a < &S::b; }
