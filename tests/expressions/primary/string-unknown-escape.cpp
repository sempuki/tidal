// tidal-rules: PRIM-35
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: unknown escape sequence
// tidal-error-gcc: unknown escape sequence
// tidal-note: Q-0330. The escape \q in a string literal is conditionally supported. Clang and GCC reject under -pedantic-errors. EDG in strict mode warns and accepts.
const char* s = "a\qb";
