// tidal-rules: CV-10
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type
// tidal-error-gcc: cannot convert
// tidal-error-edg: cannot be used to initialize an entity of type
// tidal-note: A pointer to an array of unknown bound does not convert to a pointer to an array of known bound.
int (*open)[] = nullptr;
int (*fixed)[3] = open;
