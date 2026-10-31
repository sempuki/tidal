// tidal-rules: NEW-17
// tidal-expect: reject
// tidal-error-clang: array size must be specified in new expression
// tidal-error-gcc: array with unspecified bounds
// tidal-error-edg: incomplete type .int \[\]. is not allowed
void f() { int* p = new int[]; }
