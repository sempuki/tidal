// tidal-rules: MPTR-6
// tidal-expect: reject
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
constexpr S cs{};
constexpr int S::* pm = nullptr;
constexpr int v = cs.*pm;
