// tidal-rules: PFX-13
// tidal-expect: reject
// tidal-error-clang: cannot decrement expression of type bool
// tidal-error-gcc: type .bool. in .operator--
// tidal-error-edg: bool type is not allowed
void f(bool b) { b--; }
