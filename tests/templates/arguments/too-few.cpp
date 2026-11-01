// tidal-rules: TARG-6
// tidal-expect: reject
// tidal-error-clang: too few template arguments for class template
// tidal-error-gcc: wrong number of template arguments \(1, should be 2\)
// tidal-error-edg: too few arguments for class template
template <class T, class U> struct Pair {};
Pair<int> p;
