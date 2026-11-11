// tidal-rules: PRIM-2 OPR-8
// tidal-expect: reject
// tidal-error-clang: integer literal is too large
// tidal-error-edg: integer constant is too large
// tidal-expect-gcc: accept
// tidal-note: The only literal operator for _n takes unsigned long long, and the value of the literal is one more than the largest unsigned long long on every target. The literal must have a value that unsigned long long holds. GCC accepts with a warning (Q-0440).
constexpr unsigned long long operator""_n(unsigned long long v) { return v; }
unsigned long long x = 18446744073709551616_n;
