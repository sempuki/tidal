// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-error-clang: cannot declare a class template with no name
// tidal-error-gcc: template class without a name
// tidal-error-edg: expected an identifier
template <class T> struct { T v; };
