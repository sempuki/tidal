// tidal-rules: ENUM-9
// tidal-expect: reject
// tidal-error-clang: target of using declaration conflicts with declaration already in scope
// tidal-error-gcc: Color::Red. conflicts with a previous declaration
// tidal-error-edg: conflicts with variable .Red
int Red = 0;
enum class Color { Red };
using enum Color;
