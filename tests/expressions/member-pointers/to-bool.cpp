// tidal-rules: MPTR-4
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int main() { int S::* pm = &S::a; int S::* pn = nullptr; return (pm && !pn) ? 0 : 1; }
