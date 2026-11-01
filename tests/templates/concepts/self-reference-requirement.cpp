// tidal-rules: CPT-6
// tidal-expect: reject
// tidal-error-clang: cannot refer to itself|undeclared identifier 'Loop'
// tidal-error-gcc: Loop. was not declared
// tidal-error-edg: identifier "Loop" is undefined
// tidal-note: The nested requirement inside the concept's own requires expression names the concept.
template <class T> concept Loop = requires (T t) { requires Loop<T>; };
