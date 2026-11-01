// tidal-rules: TSP-7
// tidal-expect: reject
// tidal-error-clang: explicit specialization of alias templates is not permitted
// tidal-error-gcc: expected unqualified-id before
// tidal-error-edg: an alias template cannot be explicitly specialized
template <class T> using Pointer = T*;
template <> using Pointer<int> = long*;
