// tidal-rules: UNI-5
// tidal-expect: accept
union U { int i; float f; };
constexpr U a{.f = 4.0f};
constexpr U b = a;
static_assert(b.f == 4.0f);
