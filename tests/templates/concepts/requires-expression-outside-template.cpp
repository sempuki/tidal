// tidal-rules: CPT-27
// tidal-expect: reject
// tidal-error-clang: no member named 'missing'
// tidal-error-gcc: has no member named .missing.
// tidal-error-edg: has no member "missing"
struct Plain {};
constexpr bool b = requires (Plain p) { p.missing(); };
