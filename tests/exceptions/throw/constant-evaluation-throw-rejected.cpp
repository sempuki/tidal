// tidal-rules: THR-10
// tidal-expect: reject
constexpr int check(int x) { if (x < 0) throw 1; return x; }
static_assert(check(-1) == 0);
