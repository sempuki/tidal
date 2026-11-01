// tidal-rules: CNS-26
// tidal-expect: reject
// tidal-error-clang: differs in template redeclaration
// tidal-error-gcc: different constraints
// tidal-error-edg: incompatible
// tidal-note: A and B have the same text, but they are different concepts.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept B = sizeof(T) > 1;
template <class T> requires A<T> struct Box;
template <class T> requires B<T> struct Box {};
