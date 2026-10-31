// tidal-rules: REF-6
// tidal-expect: reject
// tidal-error-clang: reference cannot be initialized with multiple values
// tidal-error-gcc: expression list treated as compound expression
// tidal-error-edg: too many initializer values
int y = 0;
int& r(y, y);
