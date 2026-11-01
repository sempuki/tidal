// tidal-rules: DEP-22
// tidal-expect: reject
// tidal-error-clang: no type named 'type' in 'Has'
// tidal-error-gcc: no type named .type. in .struct Has.
// tidal-error-edg: class "Has" has no member "type"
struct Has {};
template <class T> int read() { typename T::type value = 0; return value; }
int x = read<Has>();
