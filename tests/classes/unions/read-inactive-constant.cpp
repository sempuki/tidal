// tidal-rules: UNI-6
// tidal-expect: reject
union U { int i; float f; };
constexpr U u{.i = 1};
constexpr float x = u.f;
