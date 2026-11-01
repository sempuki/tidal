// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-error-clang: does not refer to a function template
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: no instance of function template
template <class T> requires (sizeof(T) > 1) void use(T) {}
template void use(char);
