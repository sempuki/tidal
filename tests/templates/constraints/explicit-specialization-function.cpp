// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-error-clang: no function template matches
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: no instance of function template
template <class T> requires (sizeof(T) > 1) int use(T) { return 1; }
template <> int use(char) { return 2; }
