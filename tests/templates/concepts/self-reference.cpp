// tidal-rules: CPT-6
// tidal-expect: reject
// tidal-error-clang: cannot refer to itself|undeclared identifier 'Loop'
// tidal-error-gcc: Loop. was not declared
// tidal-error-edg: identifier "Loop" is undefined
template <class T> concept Loop = sizeof(T) > 1 && Loop<T*>;
