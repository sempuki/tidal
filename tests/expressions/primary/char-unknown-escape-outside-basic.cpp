// tidal-rules: PRIM-18
// tidal-expect: reject
// tidal-error-clang: unknown escape sequence
// tidal-error-gcc: unknown escape sequence
// tidal-note: Q-0330. A backslash before a character outside the basic character set fits no escape form.
char c = '\é';
