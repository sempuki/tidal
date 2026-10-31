// tidal-rules: CAST-19
// tidal-expect: reject
// tidal-error-clang: static_cast from .double \*. to .int \*. is not allowed
// tidal-error-gcc: invalid .static_cast. from type
// tidal-error-edg: invalid type conversion
double value = 1.0;
int* view = static_cast<int*>(&value);
