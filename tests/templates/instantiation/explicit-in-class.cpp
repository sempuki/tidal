// tidal-rules: INST-21
// tidal-expect: reject
// tidal-error-clang: expected '<' after 'template'
// tidal-error-gcc: expected .<. before .int.
// tidal-error-edg: explicit instantiation is not allowed in the current scope
template <class T> int get(T) { return 1; }
struct Host { template int get(int); };
