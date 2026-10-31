// tidal-rules: DCMP-3
// tidal-expect: reject
struct P { int x; bool operator==(P) const = default; };
bool t = P{1} == P{1};
