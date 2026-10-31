// tidal-rules: DEL-2
// tidal-expect: reject
// tidal-error-clang: deleted definition must be first declaration
// tidal-error-gcc: deleted definition of .* is not first declaration
// tidal-error-edg: can only appear on the first declaration
void f();
void f() = delete;
