// tidal-rules: CTOR-3
// tidal-expect: accept
struct M { M() {} };
struct S { const M m; };
S s;
