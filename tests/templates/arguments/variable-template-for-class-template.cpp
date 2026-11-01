// tidal-rules: TARG-7
// tidal-expect: reject
// tidal-error-clang: template argument does not refer to a class or alias
// tidal-error-clang-18: requires template arguments
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-error-edg: is not a class template
template <template <class> class C> struct Use {};
template <class T> constexpr int width = sizeof(T);
Use<width> u;
