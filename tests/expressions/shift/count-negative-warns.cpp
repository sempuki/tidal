// tidal-rules: SHF-9
// tidal-expect: reject
// tidal-flags: -Werror
int f(int v) { return v << -1; }
