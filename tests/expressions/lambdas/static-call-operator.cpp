// tidal-rules: LAM-13 OVL-25
// tidal-expect: accept
auto g = [](int v) static { return v + 1; };
static_assert(g(1) == 2);
int (*p)(int) = g;
