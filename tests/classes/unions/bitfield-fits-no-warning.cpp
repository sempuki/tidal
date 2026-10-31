// tidal-rules: BF-8
// tidal-expect: accept
// tidal-flags: -Wall -Werror
struct S { unsigned x : 3; };
void f(S& s) { s.x = 7; }
