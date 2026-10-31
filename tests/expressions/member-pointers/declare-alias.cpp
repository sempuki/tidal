// tidal-rules: MPTR-1
// tidal-expect: run
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
using Field = int S::*;
int main() { Field f = &S::a; S s; return s.*f == 1 ? 0 : 1; }
