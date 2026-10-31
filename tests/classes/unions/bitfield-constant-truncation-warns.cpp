// tidal-rules: BF-8
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct S { unsigned x : 3; };
void f(S& s) { s.x = 9; }
