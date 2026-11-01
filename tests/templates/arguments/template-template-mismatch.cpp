// tidal-rules: TARG-18
// tidal-expect: reject
// tidal-error-clang: template argument for non-type template parameter must be
// tidal-error-clang-18: template template argument has different template
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-error-edg: is not compatible with template template parameter
// tidal-note: The parameter takes a template of one type, and the argument has a constant parameter.
template <int N> struct Num {};
template <template <class> class C> struct Apply {};
Apply<Num> a;
