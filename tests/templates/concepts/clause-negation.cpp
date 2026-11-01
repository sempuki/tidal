// tidal-rules: CPT-10
// tidal-expect: reject
// tidal-error-clang: parentheses are required around this expression
// tidal-error-gcc: must be enclosed in parentheses
// tidal-error-edg: invalid start of expression in requires clause
template <class T> concept Big = sizeof(T) > 1;
template <class T> requires !Big<T> void use(T);
