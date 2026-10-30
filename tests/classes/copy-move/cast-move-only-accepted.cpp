// tidal-rules: CPY-9
// tidal-expect: accept
struct M { M() {} M(M&&) {} };
void take(M) {}
void f() { M m; take(static_cast<M&&>(m)); }
