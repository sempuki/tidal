// tidal-rules: MUL-16
// tidal-expect: reject
// tidal-error-clang: different enumeration types.*deprecated
// tidal-error-gcc: different enumeration types.*deprecated
// tidal-flags: -Werror -Wdeprecated-enum-enum-conversion
enum Width { wide = 4 };
enum Height { tall = 3 };
int area() { return wide * tall; }
