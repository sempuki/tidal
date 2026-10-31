// tidal-rules: UNI-6
// tidal-expect: reject
union U { int i; float f; };
constexpr int f() { U u{.i = 1}; u.f = 2.0f; return u.i; }
static_assert(f() == 1);
