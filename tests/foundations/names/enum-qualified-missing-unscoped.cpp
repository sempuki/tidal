// tidal-rules: NAM-20
// tidal-expect: reject
// tidal-error-clang: no member named .blue. in .Color.
// tidal-error-gcc: blue. is not a member of .Color
// tidal-error-edg: enum .Color. has no member .blue.
// tidal-note: The enumeration is unscoped, and the name after :: is not one of its enumerators.
enum Color { red, green };
Color c = Color::blue;
