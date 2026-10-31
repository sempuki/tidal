// tidal-rules: MPTR-3
// tidal-expect: accept
struct S { int a = 1; int f() { return a; } int g() const { return a; } };
int h(S& s, int (S::* pf)() const) { return (s.*pf)(); }
