// tidal-rules: MPTR-3
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
struct D : S { int extra = 0; };
int main() { D d; int S::* pm = &S::b; return d.*pm == 2 ? 0 : 1; }
