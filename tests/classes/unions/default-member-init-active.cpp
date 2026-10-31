// tidal-rules: UNI-4
// tidal-expect: accept
union U { int i; float f = 1.5f; };
constexpr U u{};
static_assert(u.f == 1.5f);
