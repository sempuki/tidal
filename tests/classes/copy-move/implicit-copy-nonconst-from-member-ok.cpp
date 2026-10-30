// tidal-rules: CPY-6
// tidal-expect: accept
struct M { M() {} M(M&) {} };
struct S { M m; };
S a;
S b(a);
