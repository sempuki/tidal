// tidal-rules: COMMA-8
// tidal-expect: reject
// tidal-error-clang: expected .\].
// tidal-error-gcc: expected .\]. before .,. token
// tidal-error-edg: expected a .\].
// tidal-note: Gap review 2026-10-31 (third).
int a[1, 2];
