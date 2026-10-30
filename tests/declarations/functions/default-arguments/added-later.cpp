// tidal-rules: DEFARG-2
// tidal-expect: accept
constexpr int f(int a, int b);
constexpr int f(int a, int b = 2);
constexpr int f(int a = 1, int b);
constexpr int f(int a, int b) { return a * 10 + b; }
static_assert(f() == 12);
