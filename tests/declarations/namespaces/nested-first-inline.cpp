// tidal-rules: NS-11
// tidal-expect: reject
// tidal-error-clang: nested namespace definition cannot be .inline
// tidal-error-gcc: nested namespace definition cannot be inline
// tidal-error-edg: inline. keyword cannot be used on a nested namespace
// tidal-note: Gap review 2026-10-30.
inline namespace A::B {}
