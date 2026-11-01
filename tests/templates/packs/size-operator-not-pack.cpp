// tidal-rules: PACK-20
// tidal-expect: reject
// tidal-error-clang: does not refer to the name of a parameter pack
// tidal-error-gcc: contains no parameter packs
// tidal-error-edg: can be used only in a variadic
template <class T> constexpr auto count = sizeof...(T);
