// tidal-rules: MPTR-3
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int main() { S s; S* p = &s; int S::* pm = &S::a; p->*pm = 7; return s.a == 7 ? 0 : 1; }
