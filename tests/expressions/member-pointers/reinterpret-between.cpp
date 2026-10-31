// tidal-rules: MPTR-4
// tidal-expect: accept
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
struct T { long z; };
auto pm = reinterpret_cast<long T::*>(&S::a);
