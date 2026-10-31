// tidal-rules: PRIM-8
// tidal-expect: reject
constexpr int f() { char* p = const_cast<char*>("abc"); p[0] = 'x'; return 0; }
static_assert(f() == 0);
