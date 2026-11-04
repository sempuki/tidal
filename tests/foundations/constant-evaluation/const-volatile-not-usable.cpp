// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-clang: variable length array
// tidal-error-gcc: is not an integral constant-expression
// tidal-error-edg: attempt to read from volatile storage
// tidal-note: Gap review 2026-10-31.
const volatile int cv = 3;
int arr[cv];
