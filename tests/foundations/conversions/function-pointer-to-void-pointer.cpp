// tidal-rules: CONV-9
// tidal-expect: reject
// tidal-error-clang: type .void \*. with an lvalue of type .void \(\).
// tidal-error-gcc: invalid conversion from .void \(\*\)\(\). to .void\*.
// tidal-error-edg: void \(\*\)\(\). cannot be used to initialize an entity of type .void \*
void f();
void* v = f;
