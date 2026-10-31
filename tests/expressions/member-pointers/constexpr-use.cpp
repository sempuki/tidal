// tidal-rules: MPTR-3
// tidal-expect: accept
struct S { int a = 1; int b = 2; constexpr int f(int x) const { return a + x; } static int s; };
int S::s = 0;
constexpr S cs{};
constexpr int S::* pm = &S::b;
static_assert(cs.*pm == 2);
static_assert((cs.*&S::f)(1) == 2);
