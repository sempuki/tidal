// tidal-rules: MPTR-9
// tidal-expect: reject
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
struct T { int v; int T::* get() { return &v; } };
