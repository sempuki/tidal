// tidal-rules: DCL-20
// tidal-expect: reject
// tidal-error-clang: array has incomplete element type .int\[\]
// tidal-error-gcc: must have bounds for all dimensions except the first
// tidal-error-edg: array may not have elements of this type
int a[][] = {{1}, {2}};
