// tidal-rules: TSP-6
// tidal-expect: reject
// tidal-error-clang: after instantiation
// tidal-error-gcc: after instantiation
// tidal-error-edg: must precede its first use
template <class T> int code(T) { return 1; }
int early = code(1);
template <> int code<int>(int) { return 2; }
