// tidal-rules: CAST-2
// tidal-expect: reject
// tidal-error-clang: static_cast from .int \*. to .long \*. is not allowed
// tidal-error-gcc: invalid .static_cast. from type .int\*. to type .long int\*.
// tidal-error-edg: invalid type conversion
int x;
long* p = static_cast<long*>(&x);
