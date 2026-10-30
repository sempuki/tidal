// tidal-rules: CE-9
// tidal-expect: reject
constexpr const int* f() { int x = 1; return &x; }
constexpr int g() { return *f(); }
static_assert(g() == 1);
