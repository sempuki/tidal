// tidal-rules: LIST-2
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: invalid narrowing conversion
struct S { int a; };
S s{2.5};
