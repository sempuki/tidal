// tidal-rules: FN-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct Inc;
void take(Inc i);
Inc make();
void gone(Inc i) = delete;
