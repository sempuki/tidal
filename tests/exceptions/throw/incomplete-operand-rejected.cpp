// tidal-rules: THR-3
// tidal-expect: reject
// tidal-error-clang: cannot throw object of incomplete type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: incomplete type .I. is not allowed
struct I;
void g(I& i) { throw i; }
