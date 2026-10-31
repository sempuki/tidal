// tidal-rules: DCMP-7
// tidal-expect: accept
// tidal-flags: -Wall -Werror
struct S { int v; bool operator==(const S& o) const { return v == o.v; } };
bool f(S a, S b) { return a == b; }
