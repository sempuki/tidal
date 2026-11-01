// tidal-rules: DED-21 DED-35
// tidal-expect: reject
// tidal-error-clang: deduced conflicting types for parameter
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: no instance of function template
// tidal-note: The two arguments give T the values int and long.
template <class T> T larger(T a, T b) { return a < b ? b : a; }
long n = larger(1, 2L);
