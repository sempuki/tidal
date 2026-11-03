// tidal-rules: UNEV-4
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: side effects has no effect in an unevaluated context
// tidal-flags: -Wall -Werror
// tidal-note: Recommended practice only. The operand of decltype has an increment. Clang warns by default. GCC gives no warning.
int i = 0;
decltype(i++) j = 0;
