// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-error-clang: expected ';' after struct
// tidal-error-gcc: a class template declaration must not declare anything else
// tidal-error-edg: expected a ";"
template <class T> struct Box { T value; } box;
