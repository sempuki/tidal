// tidal-rules: CVF-11
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool
// tidal-error-gcc: invalid user-defined conversion
// tidal-error-edg: must have bool type
struct S { explicit operator int() const { return 1; } };
void f() { if (S{}) {} }
