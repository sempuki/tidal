// tidal-rules: CX-3
// tidal-expect: reject
int read() { return 1; }
constexpr int f() { return read(); }
static_assert(f() == 1);
