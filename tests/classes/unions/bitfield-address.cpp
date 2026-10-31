// tidal-rules: BF-4
// tidal-expect: reject
// tidal-error-clang: address of bit-field requested
// tidal-error-gcc: take address of bit-field
// tidal-error-edg: address of a bit field is not allowed
struct S { int x : 3; };
int* f(S& s) { return &s.x; }
