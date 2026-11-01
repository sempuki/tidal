// tidal-rules: TPL-39
// tidal-expect: reject
// tidal-error-clang: expected unqualified-id
// tidal-error-gcc: expected unqualified-id
// tidal-error-edg: invalid narrowing conversion
// tidal-note: The > after 3 ends the template head.
template <bool B = 3 > 4> struct Flag {};
