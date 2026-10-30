// tidal-rules: CPY-7
// tidal-expect: accept
struct M { M() {} M(M&&) {} };
struct S { M m; };
S a;
S b(static_cast<S&&>(a));
