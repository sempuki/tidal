// tidal-rules: UNEV-3
// tidal-expect: reject
// tidal-error-clang: expression result unused
// tidal-error-gcc: statement has no effect
// tidal-flags: -Wall -Werror
// tidal-note: The discarded expression is an addition.
int f(int x) { x + 1; return x; }
