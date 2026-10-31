// tidal-rules: UNEV-1
// tidal-expect: reject
int f() = delete;
using T = decltype(f());
