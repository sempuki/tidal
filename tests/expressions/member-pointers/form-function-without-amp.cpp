// tidal-rules: MPTR-2
// tidal-expect: reject
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int (S::* pf)(int) const = S::f;
