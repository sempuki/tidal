// tidal-rules: CVF-11
// tidal-expect: reject
// tidal-error-clang: no viable conversion
// tidal-error-gcc: cannot convert
// tidal-error-edg: no suitable conversion function
struct S { explicit operator int() const { return 1; } };
int x = S{};
