// tidal-rules: SHF-9
// tidal-expect: reject
// tidal-error-clang: shift count is negative
// tidal-error-gcc: shift count is negative
// tidal-flags: -Werror
int f(int v) { return v << -1; }
