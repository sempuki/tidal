// tidal-rules: FN-12
// tidal-expect: reject
// tidal-error-clang: incomplete type .Inc
// tidal-error-gcc: has incomplete type
// tidal-error-edg: incomplete type .Inc. is not allowed
// tidal-note: Gap review 2026-10-31.
struct Inc;
void take(Inc i) {}
