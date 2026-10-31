// tidal-rules: UNEV-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct Inc;
Inc make();
using T = decltype(make().x);
