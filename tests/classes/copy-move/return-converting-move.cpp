// tidal-rules: CPY-11
// tidal-expect: accept
struct M { M() {} M(M&&) {} };
struct H { H(M&&) {} };
H make() { M m; return m; }
