// tidal-rules: COND-15
// tidal-expect: reject
// tidal-error-gcc: call to non-.constexpr. function
// tidal-error-clang: non-constexpr function .* cannot be used in a constant expression
// tidal-error-edg: cannot call non-constexpr function
int runtime();
constexpr int f(bool b) { return b ? 1 : runtime(); }
static_assert(f(false) == 1);
