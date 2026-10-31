// tidal-rules: PFX-2
// tidal-expect: reject
// tidal-error-clang: array subscript is not an integer
// tidal-error-gcc: invalid types .int \[2\]\[double\]. for array subscript
// tidal-error-edg: must have integral or unscoped enum type
int a[2];
int f() { return a[1.0]; }
