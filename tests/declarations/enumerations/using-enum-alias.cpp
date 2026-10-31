// tidal-rules: ENUM-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
enum class Color { red, green };
using Shade = Color;
int f() { using enum Shade; return static_cast<int>(green); }
