// tidal-rules: INST-24
// tidal-expect: reject
// tidal-error-clang: that is not an instantiation
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: compiler generated function .* cannot be explicitly instantiated
template <class T> struct Box {};
template Box<int>::Box();
