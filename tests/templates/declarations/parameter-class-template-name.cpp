// tidal-rules: TPL-18
// tidal-expect: reject
// tidal-error-clang: shadows template parameter
// tidal-error-gcc: using template type parameter .T. after .struct.
// tidal-error-edg: class template and template parameter may not
template <class T> struct T {};
