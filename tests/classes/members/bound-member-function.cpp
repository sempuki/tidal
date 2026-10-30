// tidal-rules: CLS-4
// tidal-expect: reject
struct S { int f() { return 1; } };
void g(S s) { auto p = s.f; }
