// tidal-rules: TSP-4
// tidal-expect: reject
// tidal-error-clang: explicit specialization of non-template struct
// tidal-error-gcc: is not a class template
// tidal-error-edg: may not have a template argument list
struct Plain {};
template <> struct Plain<int> {};
