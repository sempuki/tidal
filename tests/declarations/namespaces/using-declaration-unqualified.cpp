// tidal-rules: NS-16
// tidal-expect: reject
// tidal-error-clang: using declaration requires a qualified name
// tidal-error-gcc: expected nested-name-specifier before .x
// tidal-error-edg: namespace-qualified name is required
int x;
using x;
