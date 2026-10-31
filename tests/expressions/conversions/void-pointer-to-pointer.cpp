// tidal-rules: CONV-9
// tidal-expect: reject
// tidal-error-clang: type .int \*. with an lvalue of type .void \*.
// tidal-error-gcc: invalid conversion from .void\*. to .int\*.
// tidal-error-edg: void \*. cannot be used to initialize an entity of type .int \*
void* v = nullptr;
int* p = v;
