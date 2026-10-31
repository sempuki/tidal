// tidal-rules: UNEV-1
// tidal-expect: accept
using L = decltype([] { return 1; });
auto n = sizeof([] { return 2; });
