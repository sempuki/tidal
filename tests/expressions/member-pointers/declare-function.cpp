// tidal-rules: MPTR-1
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int main() { int (S::* pf)(int) const = &S::f; S s; return (s.*pf)(3) == 4 ? 0 : 1; }
