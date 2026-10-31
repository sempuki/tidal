// tidal-rules: DCL-7
// tidal-expect: reject
// tidal-error-clang: function cannot return array type
// tidal-error-gcc: declared as function returning an array
// tidal-error-edg: function returning array is not allowed
int f()[3];
