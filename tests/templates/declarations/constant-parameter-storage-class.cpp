// tidal-rules: TPL-33
// tidal-expect: reject
// tidal-error-clang: invalid declaration specifier in template non-type parameter
// tidal-error-gcc: storage class specified for template parameter
// tidal-error-edg: a storage class may not be specified here
template <static int N> struct Fixed {};
