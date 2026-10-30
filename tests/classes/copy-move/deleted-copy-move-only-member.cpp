// tidal-rules: CPY-7
// tidal-expect: reject
struct M { M() {} M(M&&) {} };
struct S { M m; };
S a;
S b(a);
