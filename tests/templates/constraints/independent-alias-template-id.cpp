// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for alias template 'Alias'
// tidal-error-gcc: template constraint failure
// tidal-error-edg: template constraint not satisfied
// tidal-note: Alias<char> does not depend on T, and Holder is never instantiated.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> using Alias = T;
template <class T> struct Holder { Alias<char> member; };
