// tidal-rules: TYN-9
// tidal-expect: reject
// tidal-expect-gcc-14: accept
// tidal-error-clang: non-C-compatible type given name for linkage purposes
// tidal-error-gcc: non-C-compatible type given name for linkage purposes
// tidal-error-edg: non-C-like type given name for linkage purposes
// tidal-note: Q-0067. GCC 14 accepts the alias, and GCC 16, Clang, and EDG reject it. The unnamed class has a member type alias, which is neither a data member, a member class, nor a member enumeration.
typedef struct { using unit = int; unit v; } Reading;
