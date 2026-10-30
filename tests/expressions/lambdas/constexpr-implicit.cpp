// tidal-rules: LAM-12
// tidal-expect: accept
constexpr auto square = [](int v) { return v * v; };
static_assert(square(4) == 16);
static_assert([] { return 2; }() == 2);
