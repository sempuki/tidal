// tidal-rules: CNS-24 CNS-27
// tidal-expect: reject
// tidal-error-clang: call to 'f' is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The clauses contain lambda expressions, so the two declarations declare two templates, and neither is more constrained.
template <class T> requires ([] { return true; }()) int f(T);
template <class T> requires ([] { return true; }()) int f(T) { return 1; }
int chosen = f(1);
