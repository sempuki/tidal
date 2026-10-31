// tidal-rules: COND-3 VC-12
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: must be an lvalue
// tidal-note: Gap review 2026-10-31 (third).
const int c = 1;
volatile int v = 2;
auto p = &(true ? c : v);
