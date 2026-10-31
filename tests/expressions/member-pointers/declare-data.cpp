// tidal-rules: MPTR-1
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int main() { int S::* pm = &S::b; S s; return s.*pm == 2 ? 0 : 1; }
