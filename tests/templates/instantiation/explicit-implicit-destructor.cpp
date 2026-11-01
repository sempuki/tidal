// tidal-rules: INST-24
// tidal-expect: reject
// tidal-error-clang: that is not an instantiation
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: compiler generated function .* cannot be explicitly instantiated
// tidal-note: The destructor of Box<int> is declared by the implementation, so an explicit instantiation must not name it.
template <class T> struct Box {};
template Box<int>::~Box();
