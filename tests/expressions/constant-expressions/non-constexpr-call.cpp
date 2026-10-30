// tidal-rules: CE-2
// tidal-expect: reject
int plain() { return 1; }
constexpr int f() { return plain(); }
static_assert(f() == 1);
