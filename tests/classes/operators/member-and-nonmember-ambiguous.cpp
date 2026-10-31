// tidal-rules: OPR-6
// tidal-expect: reject
struct V { int x;  V operator+(V) const; };
V operator+(V, V);
V f(V a, V b) { return a + b; }
