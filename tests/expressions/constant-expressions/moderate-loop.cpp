// tidal-rules: CE-10
// tidal-expect: accept
// tidal-note: Write report W11. GCC stops each loop at 262144 iterations, so 200000 is under every matrix limit.
constexpr long count(long n) { long c = 0; for (long i = 0; i < n; ++i) ++c; return c; }
static_assert(count(200000) == 200000);
