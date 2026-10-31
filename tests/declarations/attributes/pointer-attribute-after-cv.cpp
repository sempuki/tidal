// tidal-rules: ATTR-18
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: an attribute list cannot appear here
// tidal-note: Q-0123. GCC and EDG accept an attribute after the cv-qualifiers of a pointer; Clang rejects it there.
int i = 0;
int * const [[]] q = &i;
int * [[]] r = &i;
