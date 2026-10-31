// tidal-rules: CV-11
// tidal-expect: reject
// tidal-error-clang: variable length array
// tidal-error-gcc: size of array .arr. is not an integral constant-expression
// tidal-error-edg: must have a constant value
// tidal-note: Gap review 2026-10-31.
const volatile int n = 1;
int arr[n];
