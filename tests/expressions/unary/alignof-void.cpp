// tidal-rules: UN-17
// tidal-expect: reject
// tidal-error-clang: alignof. to an incomplete type .void
// tidal-error-gcc: alignof. to a void type
// tidal-error-edg: applied to incomplete type
// tidal-note: Gap review 2026-10-31.
auto a = alignof(void);
