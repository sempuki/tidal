// tidal-rules: LIST-11
// tidal-expect: reject
// tidal-error-clang: too many braces around scalar initializer
// tidal-error-gcc: too many braces around scalar initializer
// tidal-error-edg: only one level of braces is allowed
// tidal-note: Gap review 2026-10-31.
int x{{1}};
