// tidal-rules: DCL-16
// tidal-expect: reject
// tidal-error-clang: pointer to function type .* cannot have .const. qualifier
// tidal-error-gcc: cannot declare pointer to qualified function type
// tidal-error-edg: pointer or reference to qualified function type
// tidal-note: Gap review 2026-10-31.
using F = void() const;
F* p;
