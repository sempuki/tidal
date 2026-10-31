// tidal-rules: UNEV-1
// tidal-expect: accept
struct Inc;
Inc make();
using T = decltype(make());
