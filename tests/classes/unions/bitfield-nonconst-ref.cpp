// tidal-rules: BF-4
// tidal-expect: reject
struct S { int x : 3; };
int& f(S& s) { return s.x; }
