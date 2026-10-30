// tidal-rules: CPY-9
// tidal-expect: reject
struct M { M() {} M(M&&) {} };
void take(M);
void f() { M m; take(m); }
