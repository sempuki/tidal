// tidal-rules: OPR-16
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: conflicts with literal operator template
// tidal-error-edg: ambiguous literal operators
// tidal-note: Q-0435. For _n, both the const char* form and a literal operator template exist, and no numeric form, so 12_n is not valid. GCC rejects the second declaration, and Clang and EDG reject the call.
template <char... Cs> constexpr int operator""_n() { return 2; }
constexpr int operator""_n(const char*) { return 1; }
int x = 12_n;
