// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: unknown escape sequence
// tidal-error-gcc: unknown escape sequence
// tidal-note: Q-0330. Clang and GCC warn, and -pedantic-errors makes the warning an error. EDG in strict mode warns and accepts.
char c = '\q';
