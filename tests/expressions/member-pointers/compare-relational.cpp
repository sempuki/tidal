// tidal-rules: MPTR-12
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression .*int S::\*
// tidal-error-gcc: invalid operands of types .int S::\*. and .int S::\*.
// tidal-error-edg: must have arithmetic, unscoped enum, or pointer type
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
bool g(int S::* p, int S::* q) { return p < q; }
