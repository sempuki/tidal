// tidal-rules: UNEV-4
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: side effects has no effect in an unevaluated context
// tidal-note: Recommended practice only. Clang warns by default. GCC gives no warning.
// tidal-flags: -Wall -Werror
int i = 0;
auto n = sizeof(i++);
