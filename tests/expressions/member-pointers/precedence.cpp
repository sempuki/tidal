// tidal-rules: MPTR-3
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int main() { S s; int S::* pm = &S::b; S* p = &s; return s.*pm * 3 == 6 && p->*pm + 1 == 3 ? 0 : 1; }
