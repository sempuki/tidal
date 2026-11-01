// tidal-rules: TPL-21
// tidal-expect: reject
// tidal-error-clang: as different kind of symbol
// tidal-error-gcc: conflicting declaration of template
// tidal-error-edg: has already been declared in the current scope
template <class T> struct Size {};
template <class T> constexpr int Size = sizeof(T);
