// tidal-rules: MUL-3
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int S::\*
// tidal-error-gcc: invalid operands of types .int S::\*. and .int.
// tidal-error-edg: must have arithmetic or unscoped enum type
// tidal-note: Gap review 2026-10-31 (third).
struct S { int m; };
auto f(int S::* p) { return p * 2; }
