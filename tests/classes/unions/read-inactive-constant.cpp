// tidal-rules: UNI-6
// tidal-expect: reject
// tidal-error-gcc: member instead of (active|initialized)
union U { int i; float f; };
constexpr U u{.i = 1};
constexpr float x = u.f;
