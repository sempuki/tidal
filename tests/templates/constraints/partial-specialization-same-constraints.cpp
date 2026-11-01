// tidal-rules: CNS-36
// tidal-expect: reject
// tidal-error-clang: not more specialized than the primary template
// tidal-error-gcc: not more constrained than the primary template
// tidal-error-edg: a template argument list is not allowed in a declaration of a primary template
template <class T> concept A = sizeof(T) > 1;
template <A T> struct Kind {};
template <A T> struct Kind<T> {};
