// tidal-rules: CE-2
// tidal-expect: reject
// tidal-error-gcc: call to non-.constexpr. function
// tidal-error-clang: non-constexpr function .* cannot be used in a constant expression
// tidal-error-edg: cannot call non-constexpr function
int plain() { return 1; }
constexpr int f() { return plain(); }
static_assert(f() == 1);
