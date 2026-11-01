// tidal-rules: DEP-19
// tidal-expect: reject
// tidal-error-clang: missing 'typename' prior to dependent type name
// tidal-error-gcc: expected .;. before .\{. token
// tidal-error-edg: is not a type name
struct Has { using type = int; };
template <class T> int take(T::type) { return 0; }
int x = take<Has>(1);
