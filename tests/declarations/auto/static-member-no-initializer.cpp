// tidal-rules: AUTO-17
// tidal-expect: reject
// tidal-error-clang: requires an initializer
// tidal-error-gcc: has no initializer
// tidal-error-edg: initializer required
// tidal-note: Gap review 2026-10-30.
struct S { static auto n; };
