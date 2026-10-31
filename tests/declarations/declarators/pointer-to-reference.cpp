// tidal-rules: DCL-17
// tidal-expect: reject
// tidal-error-clang: declared as a pointer to a reference
// tidal-error-gcc: cannot declare pointer to .int&
// tidal-error-edg: pointer to reference is not allowed
int x;
int&* p;
