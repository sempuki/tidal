// tidal-rules: UN-11
// tidal-expect: reject
// tidal-error-clang: arithmetic on a pointer to an incomplete type
// tidal-error-gcc: increment a pointer to incomplete type
// tidal-error-edg: pointer to a complete object type
// tidal-note: Gap review 2026-10-31.
struct Inc;
void f(Inc* p) { ++p; }
