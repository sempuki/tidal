// tidal-rules: BF-4
// tidal-expect: reject
// tidal-error-clang: non-const reference cannot bind to bit-field
// tidal-error-gcc: cannot bind bit-field
// tidal-error-edg: address of a bit field is not allowed
struct S { int x : 3; };
int& f(S& s) { return s.x; }
