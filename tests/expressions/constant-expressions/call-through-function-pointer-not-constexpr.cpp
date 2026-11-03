// tidal-rules: CE-2
// tidal-expect: reject
// tidal-error-clang: non-constexpr function .plain.
// tidal-error-gcc: call to non-.constexpr. function .int plain\(int\).
// tidal-error-edg: cannot call non-constexpr function "plain
// tidal-note: The pointer reaches a function that is not constexpr.
int plain(int v) { return v; }
constexpr int apply(int (*h)(int), int v) { return h(v); }
static_assert(apply(plain, 3) == 3);
