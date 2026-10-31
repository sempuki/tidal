// tidal-rules: REF-1
// tidal-expect: reject
// tidal-error-clang: non-const reference cannot bind to bit-field
// tidal-error-gcc: cannot bind bit-field
// tidal-error-edg: address of a bit field is not allowed
// tidal-note: Gap review 2026-10-30.
struct B { int v : 4; };
B b;
int& r = b.v;
