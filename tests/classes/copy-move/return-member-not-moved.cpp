// tidal-rules: CPY-11
// tidal-expect: reject
struct M { M() {} M(M&&) {} };
struct H { M m; M take() { return m; } };
