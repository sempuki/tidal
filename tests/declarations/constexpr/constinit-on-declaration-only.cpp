// tidal-rules: CINIT-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0025. If any declaration of a variable has constinit, the initializing declaration must have it.
extern constinit int y;
int y = 1;
