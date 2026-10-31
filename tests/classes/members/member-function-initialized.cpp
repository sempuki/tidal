// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: is not virtual and cannot be declared pure
// tidal-error-gcc: initializer specified for non-virtual method
// tidal-error-edg: allowed only on virtual functions
struct S { void f() = 0; };
