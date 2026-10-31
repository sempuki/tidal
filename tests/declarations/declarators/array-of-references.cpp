// tidal-rules: DCL-3
// tidal-expect: reject
// tidal-error-clang: declared as array of references
// tidal-error-gcc: as array of references
// tidal-error-edg: array of reference is not allowed
int x;
int& a[1] = {x};
