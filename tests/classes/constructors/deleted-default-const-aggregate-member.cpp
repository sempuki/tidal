// tidal-rules: CTOR-3
// tidal-expect: reject
struct M { int v; };
struct S { const M m; };
S s;
