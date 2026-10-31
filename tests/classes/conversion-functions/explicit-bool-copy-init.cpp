// tidal-rules: CVF-11
// tidal-expect: reject
// tidal-error-clang: no viable conversion
// tidal-error-gcc: cannot convert
// tidal-error-edg: no suitable conversion function
struct H { explicit operator bool() const { return true; } };
bool b = H{};
