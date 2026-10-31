// tidal-rules: MPTR-2
// tidal-expect: reject
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
auto h(S s) { return &s.f; }
