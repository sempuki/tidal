// tidal-rules: CNS-26 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The heads are not equivalent, so the two declarations are two function templates with the same constraint, and the call is ambiguous.
template <class T> concept A = sizeof(T) > 1;
template <A T> int f(T);
template <class T> requires A<T> int f(T) { return 1; }
int x = f(1);
