// tidal-rules: UNI-4
// tidal-expect: accept
union U { int i; float f; };
constexpr U u{};
static_assert(u.i == 0);
