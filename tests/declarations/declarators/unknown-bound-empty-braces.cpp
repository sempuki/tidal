// tidal-rules: DCL-5
// tidal-expect: reject
// tidal-error-clang: zero size arrays
// tidal-error-gcc: zero-size array
// tidal-error-edg: empty initializer is invalid for an array with unspecified bound
// tidal-note: Gap review 2026-10-31.
int arr[] = {};
