// tidal-rules: CINIT-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: constinit. specifier added after initialization of variable
// tidal-note: Q-0025. If any declaration of a variable has constinit, the initializing declaration must have it.
int x = 1;
extern constinit int x;
