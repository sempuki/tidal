// tidal-rules: CV-10
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: cannot initialize a variable of type
// tidal-note: Q-0191. The bound is dropped at the second level, and the first level has no const. GCC and EDG accept the conversion. Clang treats the dropped bound as it treats added const.
int (*row)[3] = nullptr;
int (**rows)[] = &row;
