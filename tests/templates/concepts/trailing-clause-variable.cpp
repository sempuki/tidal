// tidal-rules: CPT-13
// tidal-expect: reject
// tidal-error-clang: can only be used when declaring a function|expected ';' at end of declaration
// tidal-error-gcc: expected initializer before .requires.
// tidal-error-edg: a requires-clause is not allowed here
template <class T> int count requires (sizeof(T) > 1) = 0;
