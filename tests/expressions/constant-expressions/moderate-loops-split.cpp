// tidal-rules: CE-10
// tidal-expect: accept
// tidal-note: Write report W11. Four loops of 100000 iterations in one evaluation. GCC's loop limit counts each loop on its own.
constexpr long count(long n) { long c = 0; for (long i = 0; i < n; ++i) ++c; return c; }
constexpr long four() { return count(100000) + count(100000) + count(100000) + count(100000); }
static_assert(four() == 400000);
