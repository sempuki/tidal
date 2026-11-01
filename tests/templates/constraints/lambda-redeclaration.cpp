// tidal-rules: CNS-26 CNS-27
// tidal-expect: reject
// tidal-error-clang: requires clause differs in template redeclaration
// tidal-error-gcc: with different constraints
// tidal-error-edg: requires-clause incompatible with class template
// tidal-note: Each lambda expression is a different expression, so the two clauses are not equivalent.
template <class T> requires ([] { return true; }()) struct Box;
template <class T> requires ([] { return true; }()) struct Box {};
