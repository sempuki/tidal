// tidal-rules: CPY-6
// tidal-expect: reject
struct M { M() {} M(M&) {} };
struct S { M m; };
const S a;
S b(a);
