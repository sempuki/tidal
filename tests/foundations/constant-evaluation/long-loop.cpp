// tidal-rules: CE-10
// tidal-expect: reject
// tidal-error-gcc: loop iteration count exceeds limit
// tidal-error-clang: maximum step limit
// tidal-error-edg: excessive constexpr function call complexity
constexpr long count(long n) { long c = 0; for (long i = 0; i < n; ++i) ++c; return c; }
static_assert(count(100000000) == 100000000);
