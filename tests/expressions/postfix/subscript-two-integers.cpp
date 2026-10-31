// tidal-rules: PFX-2
// tidal-expect: reject
// tidal-error-clang: subscripted value is not an array, pointer, or vector
// tidal-error-gcc: invalid types .int\[int\]. for array subscript
// tidal-error-edg: must have pointer-to-object type
int f(int a, int b) { return a[b]; }
