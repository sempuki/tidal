// tidal-rules: CPY-11
// tidal-expect: reject
struct M { M() {} M(M&&) {} };
M take(M& m) { return m; }
