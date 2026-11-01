// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed to type
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: : constant value does not fit in destination type
// tidal-note: 300 does not fit in unsigned char.
template <unsigned char N> struct Byte {};
Byte<300> b;
