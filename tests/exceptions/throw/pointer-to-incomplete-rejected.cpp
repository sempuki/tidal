// tidal-rules: THR-3
// tidal-expect: reject
// tidal-error-clang: cannot throw pointer to object of incomplete type
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type
struct I;
void g(I* p) { throw p; }
