// tidal-rules: LIST-5
// tidal-expect: reject
// tidal-error-clang: array designators are a C99 extension
// tidal-error-gcc: does not allow C99 designated initializers
// tidal-error-edg: array designators are nonstandard
int a[3] = {[1] = 5};
