// tidal-rules: OPR-8 PRIM-15
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: floating constant is out of range
// tidal-allows-edg: PRIM-15
// tidal-note: Ledger row F6-1. The long double form receives the value of 1e5000L, which is outside the range of long double, so PRIM-15 applies. Clang and GCC accept the literal. EDG rejects it, as it rejects 1e5000L (Q-0063).
constexpr long double operator""_ld(long double v) { return v; }
long double huge = 1e5000_ld;
