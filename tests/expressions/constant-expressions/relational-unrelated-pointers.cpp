// tidal-rules: CE-15
// tidal-expect: reject
// tidal-error-gcc: \(& a\) < \(& b\)\). is not a constant expression
// tidal-error-clang: comparison between .* has unspecified value
// tidal-error-edg: pointer values cannot be compared
// tidal-note: Gap review 2026-10-31.
int a, b;
constexpr bool lt = &a < &b;
