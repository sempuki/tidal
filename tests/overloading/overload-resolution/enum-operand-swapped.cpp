// tidal-rules: OVL-8
// tidal-expect: accept
// tidal-note: No parameter of operator==(Shade, Hue) has the type of the operand in its written position. In the swapped candidate Shade::dark == Hue::red, each parameter meets an operand of its own type, so the swapped candidate exists and wins.
enum class Hue { red };
enum class Shade { dark };
constexpr bool operator==(Shade, Hue) { return true; }
static_assert(Hue::red == Shade::dark);
