// tidal-rules: MPTR-6
// tidal-expect: reject
// tidal-error-gcc: \(& cs\)\) \+ -1\). is not a constant expression
// tidal-error-edg: dereference a null pointer-to-member
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
constexpr S cs{};
constexpr int S::* pm = nullptr;
constexpr int v = cs.*pm;
