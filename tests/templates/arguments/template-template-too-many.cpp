// tidal-rules: TARG-18
// tidal-expect: reject
// tidal-error-clang: too few template arguments for class template
// tidal-error-clang-18: template template argument has different template
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-error-edg: is not compatible with template template parameter
// tidal-note: The argument needs two arguments, and the parameter supplies one.
template <class T, class U> struct Pair {};
template <template <class> class C> struct Apply {};
Apply<Pair> a;
