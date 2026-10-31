// tidal-rules: UNEV-1
// tidal-expect: reject
struct Inc;
Inc make();
auto n = sizeof(make());
