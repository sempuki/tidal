// tidal-rules: DCMP-7
// tidal-expect: reject
// tidal-flags: -Wall -Werror
struct S { int v; bool operator==(const S& o) { return v == o.v; } };
bool f(S a, S b) { return a == b; }
