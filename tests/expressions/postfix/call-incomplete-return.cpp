// tidal-rules: PFX-14
// tidal-expect: reject
// tidal-error-clang: with incomplete return type
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: returns incomplete type
// tidal-note: Gap review 2026-10-31.
struct Inc;
Inc make();
void g() { make(); }
