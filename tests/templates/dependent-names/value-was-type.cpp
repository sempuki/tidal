// tidal-rules: DEP-19
// tidal-expect: reject
// tidal-error-clang: missing 'typename' prior to dependent type name
// tidal-error-gcc: is parsed as a non-type, but instantiation yields a type
// tidal-error-edg: type name is not allowed
struct Has { using count = int; };
template <class T> int read() { return T::count; }
int x = read<Has>();
