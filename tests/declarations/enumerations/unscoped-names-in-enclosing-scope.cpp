// tidal-rules: ENUM-2
// tidal-expect: accept
enum Color { Red, Green };
Color c = Green;
Color d = Color::Red;
static_assert(Green == 1);
