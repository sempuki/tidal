// tidal-rules: OPR-8
// tidal-expect: accept
// tidal-note: Q-0435. For _n, both the unsigned long long form and a literal operator template exist, so 12_n calls the numeric form. For _g, both the long double form and a template exist, so 1.5_g calls the numeric form. Every compiler agrees.
template <char... Cs> constexpr int operator""_n() { return 2; }
constexpr int operator""_n(unsigned long long) { return 1; }
template <char... Cs> constexpr int operator""_g() { return 2; }
constexpr int operator""_g(long double) { return 1; }
static_assert(12_n == 1 && 1.5_g == 1);
