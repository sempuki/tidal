// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct P { int a; int b; };
void f() { volatile auto [x, y] = P{1, 2}; (void)x; }
