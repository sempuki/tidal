// tidal-rules: CPY-11
// tidal-expect: accept
struct M { M() {} M(M&&) {} };
M make() { M m; return m; }
