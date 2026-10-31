// tidal-rules: MPTR-5
// tidal-expect: reject
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
bool g(int S::* p, int S::* q) { return p < q; }
