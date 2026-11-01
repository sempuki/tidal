// tidal-rules: CPT-13
// tidal-expect: reject
// tidal-error-clang: can only be used when declaring a function
// tidal-error-gcc: expected .,. or .\.\.\.. before .requires.
// tidal-error-edg: a requires-clause is not allowed here
template <class T> void use(int x requires true);
