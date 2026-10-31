// tidal-rules: CAST-2
// tidal-expect: reject
// tidal-error-clang: cannot cast from type .int. to pointer type
// tidal-error-gcc: invalid .static_cast. from type .int. to type .int\*.
// tidal-error-edg: invalid type conversion
// tidal-note: Gap review 2026-10-31.
int* p = static_cast<int*>(100);
