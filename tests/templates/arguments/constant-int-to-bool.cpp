// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: : constant value does not fit in destination type
template <bool B> struct Flag {};
Flag<2> f;
