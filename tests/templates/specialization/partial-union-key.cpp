// tidal-rules: TSP-32
// tidal-expect: reject
// tidal-error-clang: tag type that does not match previous declaration
// tidal-error-gcc: .union. tag used in naming
// tidal-error-edg: tag kind of union is incompatible with
template <class T> struct Box {};
template <class T> union Box<T*> { int i; };
