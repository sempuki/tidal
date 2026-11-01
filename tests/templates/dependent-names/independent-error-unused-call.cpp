// tidal-rules: DEP-30
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: no matching function for call to 'take'
// tidal-error-edg: incompatible with parameter of type "int \*"
// tidal-note: The template is never instantiated, and the call fails for every argument. Clang and EDG diagnose the definition, GCC does not. DEP-30 allows both (Q-0223).
void take(int*);
template <class T> void make() { take(1); }
