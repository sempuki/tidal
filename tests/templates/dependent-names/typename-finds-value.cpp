// tidal-rules: DEP-22
// tidal-expect: reject
// tidal-error-clang: typename specifier refers to non-type member
// tidal-error-gcc: which is not a type
// tidal-error-edg: is not a type name
struct Has { static constexpr int type = 0; };
template <class T> int read() { typename T::type value = 0; return value; }
int x = read<Has>();
