// tidal-rules: UNI-4
// tidal-expect: accept
union U { int i; float f; };
constexpr U u{.f = 2.0f};
static_assert(u.f == 2.0f);
