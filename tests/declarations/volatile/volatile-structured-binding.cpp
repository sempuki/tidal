// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-error-clang: volatile qualifier in structured binding declaration is deprecated
// tidal-error-gcc: volatile.-qualified structured binding is deprecated
// tidal-flags: -Wall -Werror
struct P { int a; int b; };
void f() { volatile auto [x, y] = P{1, 2}; (void)x; }
