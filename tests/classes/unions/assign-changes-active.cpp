// tidal-rules: UNI-5
// tidal-expect: accept
union U { int i; float f; };
constexpr float f() { U u{.i = 1}; u.f = 3.0f; return u.f; }
static_assert(f() == 3.0f);
