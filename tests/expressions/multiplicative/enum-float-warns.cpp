// tidal-rules: MUL-16
// tidal-expect: reject
// tidal-error-clang: enumeration type .* and floating-point type .* is deprecated
// tidal-error-gcc: enumeration type .* and floating-point type .* is deprecated
// tidal-flags: -Werror -Wdeprecated-enum-float-conversion
// tidal-note: Gap review 2026-10-30.
enum E { A = 2 };
double f() { return A * 1.5; }
