// tidal-rules: PFX-13
// tidal-expect: reject
// tidal-error-clang: incrementing expression of type bool
// tidal-error-gcc: type .bool. in .operator\+\+
// tidal-error-edg: incrementing a bool value
void f(bool b) { b++; }
