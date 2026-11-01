// tidal-rules: DEP-19
// tidal-expect: reject
// tidal-error-clang: missing 'typename' prior to dependent type name
// tidal-error-gcc: need .typename. before .T::type.
// tidal-error-edg: expected a ";"
struct Has { using type = int; };
template <class T> int read() { T::type value = 0; return value; }
int x = read<Has>();
