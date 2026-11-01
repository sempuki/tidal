// tidal-rules: CNS-13
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied for
// tidal-error-gcc: template constraint failure|invalid variable template|placeholder constraints
// tidal-error-edg: template constraint not satisfied
template <class T> requires (sizeof(T) > 1) struct Box {};
template <> struct Box<char> {};
