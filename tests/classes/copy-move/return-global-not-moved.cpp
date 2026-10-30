// tidal-rules: CPY-11
// tidal-expect: reject
struct M { M() {} M(M&&) {} };
M g;
M take() { return g; }
