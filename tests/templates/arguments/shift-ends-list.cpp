// tidal-rules: TARG-4
// tidal-expect: reject
// tidal-error-clang: expected unqualified-id
// tidal-error-gcc: expected unqualified-id before
// tidal-error-edg: expected an identifier
// tidal-note: The >> closes the list and one more list that is not open.
template <int N> struct Num {};
Num<16 >> 2> num;
