// tidal-rules: CINIT-3
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: constinit. specifier added after initialization of variable
// tidal-note: Q-0025. The constinit comes after the initializing declaration. Clang rejects, GCC and EDG accept. CINIT-3 needs no diagnostic, so all three results conform.
int x = 1;
extern constinit int x;
