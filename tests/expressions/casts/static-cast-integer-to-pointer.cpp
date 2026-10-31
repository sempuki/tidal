// tidal-rules: CAST-19
// tidal-expect: reject
// tidal-error-clang: cannot cast from type .int. to pointer type
// tidal-error-gcc: invalid .static_cast. from type
// tidal-error-edg: invalid type conversion
int* address = static_cast<int*>(42);
