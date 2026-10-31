// tidal-rules: ENUM-20
// tidal-expect: reject
// tidal-error-clang: S. is not an enumerated type
// tidal-error-gcc: using enum. of non-enumeration type
// tidal-error-edg: is not an enum type
struct S {};
using enum S;
