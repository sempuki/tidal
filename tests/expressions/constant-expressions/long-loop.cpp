// tidal-rules: CE-10
// tidal-expect: reject
constexpr long count(long n) { long c = 0; for (long i = 0; i < n; ++i) ++c; return c; }
static_assert(count(100000000) == 100000000);
