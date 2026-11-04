// tidal-rules: DEFARG-13
// tidal-expect: reject
// tidal-error-clang: default arguments cannot be added to a function template
// tidal-error-gcc: may not have default arguments
// tidal-error-edg: default argument
// tidal-note: A later declaration of a function template must not add a default argument.
template <class T> void place(T, int);
template <class T> void place(T, int = 1);
