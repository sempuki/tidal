// tidal-rules: UNI-6
// tidal-expect: reject
union U { int i; long l; };
constexpr int f() { U u{.i = 1}; u.l += 1; return 0; }
static_assert(f() == 0);
