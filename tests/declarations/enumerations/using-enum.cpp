// tidal-rules: ENUM-9
// tidal-expect: accept
enum class Color { Red, Green };
using enum Color;
Color c = Green;
static_assert(Red != Green);
