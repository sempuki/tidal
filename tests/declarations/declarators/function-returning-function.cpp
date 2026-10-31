// tidal-rules: DCL-7
// tidal-expect: reject
// tidal-error-clang: function cannot return function type
// tidal-error-gcc: declared as function returning a function
// tidal-error-edg: function returning function is not allowed
int f()();
