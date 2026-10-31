// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-error-clang: invalid literal operator parameter type
// tidal-error-gcc: has invalid argument list
// tidal-error-edg: invalid integer parameter type .* for literal operator
int operator""_bad(int);
