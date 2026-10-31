// tidal-rules: PFX-14
// tidal-expect: reject
// tidal-error-clang: argument type .Inc. is incomplete
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: cannot convert to incomplete class
// tidal-note: Gap review 2026-10-31.
struct Inc;
void take(Inc);
void g(Inc& i) { take(i); }
