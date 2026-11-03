// tidal-rules: PRIM-15
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: floating constant is out of range
// tidal-note: Q-0063. A double literal with no suffix. Clang and GCC accept and give infinity. EDG 7.0 rejects it.
double x = 1e400;
