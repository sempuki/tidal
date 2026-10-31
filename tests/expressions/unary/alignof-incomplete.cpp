// tidal-rules: UN-17
// tidal-expect: reject
// tidal-error-clang: alignof. to an incomplete type
// tidal-error-gcc: alignof__. to incomplete type
// tidal-error-edg: applied to incomplete type
// tidal-note: Gap review 2026-10-31.
struct Inc;
auto a = alignof(Inc);
