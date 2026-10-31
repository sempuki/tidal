// tidal-rules: MPTR-3
// tidal-expect: reject
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
void g(const S& s, int S::* pm) { s.*pm = 1; }
