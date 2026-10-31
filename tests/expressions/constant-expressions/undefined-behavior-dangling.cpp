// tidal-rules: CE-9
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: read of variable whose lifetime has ended
// tidal-error-edg: attempt to access expired storage
constexpr const int* f() { int x = 1; return &x; }
constexpr int g() { return *f(); }
static_assert(g() == 1);
