// tidal-rules: CE-9
// tidal-expect: reject
// tidal-error-gcc: division by zero is not a constant expression
// tidal-error-clang: division by zero
// tidal-error-edg: division by zero
constexpr int f(int a) { return a / 0; }
static_assert(f(1) == 0);
