// tidal-rules: TARG-4
// tidal-expect: reject
// tidal-error-clang: expected unqualified-id
// tidal-error-gcc: expected unqualified-id before numeric constant
// tidal-error-edg: expected an identifier
// tidal-note: The first > ends the list, so 2> is left over.
template <int N> struct Num {};
Num<3 > 2> num;
