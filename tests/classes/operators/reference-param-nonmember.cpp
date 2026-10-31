// tidal-rules: OPR-1
// tidal-expect: accept
struct V { int x; };
bool operator==(const V& a, const V& b) { return a.x == b.x; }
