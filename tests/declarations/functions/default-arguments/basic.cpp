// tidal-rules: DEFARG-1
// tidal-expect: accept
constexpr int f(int a, int b = 2, int c = 3) { return a + b + c; }
static_assert(f(1) == 6 && f(1, 1) == 5 && f(1, 1, 1) == 3);
