// tidal-rules: CE-16
// tidal-expect: reject
// tidal-error-gcc: used before its definition
// tidal-error-clang: undefined function .* cannot be used in a constant expression
// tidal-error-edg: constexpr function .later. .* is not defined
// tidal-note: Gap review 2026-10-31.
constexpr int later();
constexpr int v = later();
constexpr int later() { return 1; }
