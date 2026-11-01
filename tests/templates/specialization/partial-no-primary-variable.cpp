// tidal-rules: TSP-31
// tidal-expect: reject
// tidal-error-clang: no variable template matches partial specialization
// tidal-error-gcc: expected initializer before
// tidal-error-edg: size is not a template
// tidal-note: The partial specialization of size comes before any declaration of a primary template.
template <class T> constexpr int size<T*> = 1;
