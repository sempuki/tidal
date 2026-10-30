// tidal-rules: CPY-11
// tidal-expect: accept
struct M { M() {} M(M&&) {} };
M pass(M m) { return m; }
