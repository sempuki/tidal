// tidal-rules: DEP-10
// tidal-expect: reject
// tidal-error-clang: neither visible in the template definition nor found by argument-dependent lookup
// tidal-error-gcc: no declarations were found by argument-dependent lookup
// tidal-error-edg: identifier "probe" is undefined
template <class T> int use(T value) { return probe(value); }
int probe(int) { return 7; }
int x = use(0);
