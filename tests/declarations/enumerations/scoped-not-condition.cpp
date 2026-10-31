// tidal-rules: ENUM-11
// tidal-expect: reject
// tidal-error-clang: not contextually convertible to .bool
// tidal-error-gcc: could not convert .S::on. from .S. to .bool
// tidal-error-edg: must have bool type
// tidal-note: Gap review 2026-10-31.
enum class S { off, on };
int f() { if (S::on) return 1; return 0; }
