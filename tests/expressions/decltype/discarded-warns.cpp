// tidal-rules: UNEV-3
// tidal-expect: reject
// tidal-error-clang: equality comparison result unused
// tidal-error-gcc: statement has no effect
// tidal-flags: -Wall -Werror
int f(int x) { x == 1; return x; }
