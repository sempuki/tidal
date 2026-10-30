// tidal-rules: ENUM-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
enum class C { R, G };
using C::R;
C pick() { return R; }
