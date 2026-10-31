// tidal-rules: MPTR-3
// tidal-expect: reject
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
int h(S s) { return s.*5; }
