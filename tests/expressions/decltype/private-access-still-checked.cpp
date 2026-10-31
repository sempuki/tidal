// tidal-rules: UNEV-1
// tidal-expect: reject
class P { int m; };
using T = decltype(P().m);
