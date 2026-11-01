// tidal-rules: TARG-7
// tidal-expect: reject
// tidal-error-clang: template argument for template template parameter must be a
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-error-edg: is not a class template
template <template <class> class C> struct Use {};
struct Plain {};
Use<Plain> u;
