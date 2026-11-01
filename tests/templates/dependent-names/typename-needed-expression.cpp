// tidal-rules: DEP-19
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: missing 'typename' prior to dependent type name
// tidal-error-gcc: is parsed as a non-type, but instantiation yields a type
// tidal-note: T::type is read as an expression, and the instantiation makes it a type. Clang and GCC reject the instantiation, EDG takes the type (Q-0224).
struct Has { using type = int; };
template <class T> int size() { return sizeof(T::type); }
int x = size<Has>();
