// tidal-rules: CTOR-3
// tidal-expect: reject
struct M { M(int); };
struct S { M m; };
S s;
