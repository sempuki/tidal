// tidal-rules: LIST-12
// tidal-expect: reject
// tidal-error-clang: excess elements in struct initializer
// tidal-error-gcc: too many initializers for .S
// tidal-error-edg: too many initializer values
struct S { int a; };
S s{1, 2};
