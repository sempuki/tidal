// tidal-rules: BF-4
// tidal-expect: reject
// tidal-error-clang: sizeof. to bit-field
// tidal-error-gcc: sizeof. to a bit-field
// tidal-error-edg: operand of sizeof may not be a bit field
struct S { int x : 3; };
auto n = sizeof(S{}.x);
