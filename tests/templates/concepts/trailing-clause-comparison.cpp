// tidal-rules: CPT-10
// tidal-expect: reject
// tidal-error-clang: parentheses are required around this expression
// tidal-error-gcc: must be enclosed in parentheses
// tidal-error-edg: invalid start of expression in requires clause
template <class T> void use(T) requires sizeof(T) > 1;
