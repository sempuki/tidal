// tidal-rules: CMP-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: suggest parentheses around comparison in operand of
// tidal-flags: -Werror -Wparentheses
// tidal-note: Q-0029. CMP-14 covers the relational operators only. GCC also warns about an equality chain. Clang gives no warning.
bool f(int a, int b, int c) { return a == b == c; }
