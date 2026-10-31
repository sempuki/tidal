// tidal-rules: OPR-8
// tidal-expect: accept
constexpr unsigned long long operator""_k(unsigned long long n) { return n * 1000; }
static_assert(4_k == 4000);
