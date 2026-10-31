// tidal-rules: LIST-5
// tidal-expect: reject
// tidal-error-clang: initializer overrides prior initialization of this subobject
// tidal-error-gcc: designator used multiple times
// tidal-error-edg: duplicate designator is not allowed
struct S { int a; int b; };
S s{.a = 1, .a = 2};
