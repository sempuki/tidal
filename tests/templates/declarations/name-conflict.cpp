// tidal-rules: TPL-21
// tidal-expect: reject
// tidal-error-clang: as different kind of symbol
// tidal-error-gcc: redeclared as different kind of entity
// tidal-error-edg: has already been declared in the current scope
template <class T> struct Box {};
int Box = 0;
