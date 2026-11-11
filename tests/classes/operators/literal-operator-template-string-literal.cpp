// tidal-rules: OPR-16 OPR-8
// tidal-expect: reject
// tidal-error-clang: no matching literal operator
// tidal-error-gcc: no matching function for call
// tidal-error-edg: user-defined literal
// tidal-note: Q-0435. A literal operator template with a pack of char takes only integer and floating literals. "ab"_n has no string form of _n to call.
template <char... Cs> constexpr int operator""_n() { return sizeof...(Cs); }
int x = "ab"_n;
