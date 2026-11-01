// tidal-rules: DED-30 DED-35
// tidal-expect: reject
// tidal-error-clang: does not match required type
// tidal-error-gcc: no matches converting function
// tidal-error-edg: matches the required type
// tidal-note: No value of T makes Tag<T>(T) the type void(int).
template <class T> struct Tag {};
template <class T> Tag<T> take(T);
void (*p)(int) = take;
