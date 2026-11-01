// tidal-rules: INST-23
// tidal-expect: reject
// tidal-error-clang: explicit template instantiation|cannot be defined in an explicit instantiation
// tidal-error-gcc: expected .;. before .,. token
// tidal-error-edg: expected a ";"
template <class T> int get(T) { return 1; }
template int get(int), get(long);
