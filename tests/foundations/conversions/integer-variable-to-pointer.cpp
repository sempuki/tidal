// tidal-rules: CONV-8
// tidal-expect: reject
// tidal-error-clang: type .int \*. with an lvalue of type .int.
// tidal-error-gcc: invalid conversion from .int. to .int\*.
// tidal-error-edg: int. cannot be used to initialize an entity of type .int \*
int zero = 0;
int* p = zero;
