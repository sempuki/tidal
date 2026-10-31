// tidal-rules: PFX-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct Inc;
void take(Inc);
void g(Inc& i) { take(i); }
