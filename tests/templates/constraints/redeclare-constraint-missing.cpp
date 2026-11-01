// tidal-rules: CNS-26
// tidal-expect: reject
// tidal-error-clang: differs in template redeclaration
// tidal-error-gcc: different constraints
// tidal-error-edg: incompatible
template <class T> concept A = sizeof(T) > 1;
template <A T> struct Box;
template <class T> struct Box {};
