// tidal-rules: TARG-6 TARG-20
// tidal-expect: reject
// tidal-error-clang: too many template arguments for function template
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
template <class T> T identity(T v) { return v; }
int x = identity<int, int>(1);
