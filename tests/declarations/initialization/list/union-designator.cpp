// tidal-rules: LIST-5
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
union U { int i; float f; };
constexpr U u{.f = 1.5f};
static_assert(u.f == 1.5f);
constexpr U v{7};
static_assert(v.i == 7);
