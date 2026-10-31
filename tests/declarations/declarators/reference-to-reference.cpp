// tidal-rules: DCL-3
// tidal-expect: reject
// tidal-error-clang: declared as a reference to a reference
// tidal-error-gcc: cannot declare reference to .int&
// tidal-error-edg: reference to reference is not allowed
int x;
int& & r = x;
