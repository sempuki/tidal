// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for class template 'Box'
// tidal-error-gcc: template constraint failure
// tidal-error-edg: template constraint not satisfied
// tidal-note: Box<char> does not depend on T, and Holder is never instantiated.
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box {};
template <class T> struct Holder { void fill() { Box<char> box; } };
