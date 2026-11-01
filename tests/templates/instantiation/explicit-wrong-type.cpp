// tidal-rules: INST-22
// tidal-expect: reject
// tidal-error-clang: does not refer to a function template
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: no instance of function template "get" matches the specified type
template <class T> int get(T) { return 1; }
template long get(int);
