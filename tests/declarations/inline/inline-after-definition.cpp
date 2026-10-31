// tidal-rules: INL-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: inline declaration of .f. follows non-inline definition
// tidal-error-edg: cannot be declared inline after its definition
// tidal-note: Q-0068. GCC accepts inline on a declaration after the definition. Clang and EDG reject it.
int f() { return 1; }
inline int f();
