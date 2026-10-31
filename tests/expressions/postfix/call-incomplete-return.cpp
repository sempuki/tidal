// tidal-rules: PFX-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct Inc;
Inc make();
void g() { make(); }
