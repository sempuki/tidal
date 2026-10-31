// tidal-rules: MPTR-1
// tidal-expect: accept
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
const int S::* pc = &S::a;
int S::* const cp = &S::a;
