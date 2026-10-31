// tidal-rules: TR-8
// tidal-expect: reject
// tidal-error-gcc: division by zero is not a constant expression
constexpr int f() { int ignored = 1 / 0; return 5; }
static_assert(f() == 5);
