// tidal-rules: DEP-30
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: cannot initialize a variable of type 'int \*'
// tidal-error-edg: cannot be used to initialize an entity of type "int \*"
// tidal-note: The template is never instantiated, and the initialization fails for every argument. Clang and EDG diagnose the definition, GCC does not. DEP-30 allows both (Q-0223).
template <class T> void make() { int* p = 1; }
