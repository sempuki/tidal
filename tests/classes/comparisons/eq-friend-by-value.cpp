// tidal-rules: DCMP-1
// tidal-expect: accept
struct P { int x; friend bool operator==(P, P) = default; };
bool t = P{1} == P{1};
