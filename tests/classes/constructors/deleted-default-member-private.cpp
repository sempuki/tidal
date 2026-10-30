// tidal-rules: CTOR-3
// tidal-expect: reject
class M { M(); };
struct S { M m; };
S s;
