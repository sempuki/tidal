// tidal-rules: CMP-21
// tidal-expect: reject
// tidal-error-clang: enumeration type .* with floating-point type .* is deprecated
// tidal-error-gcc: enumeration type .* with floating-point type .* is deprecated
// tidal-flags: -Werror
// tidal-note: Gap review 2026-10-31 (third).
enum E { a };
bool f(double d) { return a < d; }
