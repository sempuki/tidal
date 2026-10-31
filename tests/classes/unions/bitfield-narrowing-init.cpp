// tidal-rules: BF-11
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
struct S { unsigned x : 3; };
int v = 3;
S s{v};
