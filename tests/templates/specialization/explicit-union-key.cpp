// tidal-rules: TSP-32
// tidal-expect: reject
// tidal-error-clang: tag type that does not match previous declaration
// tidal-error-gcc: .union. tag used in naming
// tidal-error-edg: invalid reference to class template
template <class T> struct Box {};
template <> union Box<int> { int i; };
