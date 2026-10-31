// tidal-rules: DEL-2
// tidal-expect: reject
// tidal-error-clang: redefinition of .f
// tidal-error-gcc: redefinition of .void f\(\)
// tidal-error-edg: has already been defined
void f() = delete;
void f() {}
