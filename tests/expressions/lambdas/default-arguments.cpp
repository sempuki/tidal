// tidal-rules: LAM-1
// tidal-expect: accept
auto g = [](int v = 4) { return v; };
static_assert(g() == 4);
