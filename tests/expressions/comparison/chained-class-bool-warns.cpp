// tidal-rules: CMP-14
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: do not have their mathematical meaning
// tidal-flags: -Werror -Wparentheses
// tidal-note: Q-0029. The inner comparison calls an operator function that returns bool. GCC warns. Clang gives no warning for class operands.
struct Level { int v; Level(int x) : v(x) {} };
bool operator<(Level a, Level b) { return a.v < b.v; }
bool f(Level a, Level b, Level c) { return a < b < c; }
