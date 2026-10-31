// tidal-rules: LOG-14
// tidal-expect: reject
// tidal-error-gcc: call to non-.constexpr. function
// tidal-error-clang: non-constexpr function .* cannot be used in a constant expression
// tidal-error-edg: cannot call non-constexpr function
int runtime_only();
constexpr bool h() { return true && runtime_only(); }
static_assert(h());
