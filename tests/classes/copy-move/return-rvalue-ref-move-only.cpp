// tidal-rules: CPY-11
// tidal-expect: accept
struct M { M() {} M(M&&) {} };
M take(M&& m) { return m; }
