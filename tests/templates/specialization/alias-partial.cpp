// tidal-rules: TSP-7
// tidal-expect: reject
// tidal-error-clang: partial specialization of alias templates is not permitted
// tidal-error-gcc: expected .=. before .<. token
// tidal-error-edg: a template-id is not allowed
template <class T> using Pointer = T*;
template <class T> using Pointer<T*> = T*;
