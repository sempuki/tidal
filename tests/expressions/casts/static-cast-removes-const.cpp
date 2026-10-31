// tidal-rules: CAST-2
// tidal-expect: reject
// tidal-error-clang: static_cast from .const int \*. to .int \*. is not allowed
// tidal-error-gcc: invalid .static_cast. from type .const int\*.
// tidal-error-edg: cannot cast away const
const int c = 1;
int* p = static_cast<int*>(&c);
