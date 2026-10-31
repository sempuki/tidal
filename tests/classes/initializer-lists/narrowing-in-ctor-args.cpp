// tidal-rules: ILC-4
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion
// tidal-error-edg: narrowing conversion
struct S { S(int) {} };
S s{1.5};
