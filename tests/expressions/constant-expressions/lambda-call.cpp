// tidal-rules: CE-1
// tidal-expect: accept
constexpr auto twice = [](int v) { return v * 2; };
static_assert(twice(4) == 8);
static_assert([] { return 5; }() == 5);
