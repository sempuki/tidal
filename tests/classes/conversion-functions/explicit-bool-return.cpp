// tidal-rules: CVF-3
// tidal-expect: reject
// tidal-error-clang: no viable conversion
// tidal-error-gcc: cannot convert
// tidal-error-edg: no suitable conversion function
struct H { explicit operator bool() const { return true; } };
bool f() { return H{}; }
