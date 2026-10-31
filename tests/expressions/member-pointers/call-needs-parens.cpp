// tidal-rules: MPTR-3
// tidal-expect: reject
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int g(S s, int (S::* pf)(int) const) { return s.*pf(1); }
