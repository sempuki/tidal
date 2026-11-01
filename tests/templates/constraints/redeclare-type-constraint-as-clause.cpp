// tidal-rules: CNS-26
// tidal-expect: reject
// tidal-error-clang: differs in template redeclaration
// tidal-error-gcc: different constraints
// tidal-error-edg: incompatible|requires-clause
// tidal-note: The constraint is the same, but one declaration writes it before the parameter and the other in a clause.
template <class T> concept A = sizeof(T) > 1;
template <A T> struct Box;
template <class T> requires A<T> struct Box {};
