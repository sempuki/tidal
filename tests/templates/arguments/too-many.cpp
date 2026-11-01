// tidal-rules: TARG-6
// tidal-expect: reject
// tidal-error-clang: too many template arguments for class template
// tidal-error-gcc: wrong number of template arguments \(2, should be 1\)
// tidal-error-edg: too many arguments for class template
template <class T> struct Box {};
Box<int, int> b;
