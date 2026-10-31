// tidal-rules: LIST-14
// tidal-expect: reject
// tidal-error-clang: mixture of designated and non-designated initializers
// tidal-error-gcc: either all initializer clauses should be designated or none
// tidal-error-edg: mixing designated and non-designated initializers
struct S { int a; int b; };
S s{.a = 1, 2};
