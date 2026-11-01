// tidal-rules: ADD-16
// tidal-expect: reject
// tidal-error-clang: floating-point type .* and enumeration type .* is deprecated
// tidal-error-gcc: floating-point type .* and enumeration type .* is deprecated
// tidal-flags: -Werror -Wdeprecated-enum-float-conversion
enum Step { one = 1 };
double next(double x) { return x + one; }
