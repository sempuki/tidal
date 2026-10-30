// tidal-rules: LIST-1
// tidal-expect: accept
constexpr int a{5};
constexpr int b = {5};
constexpr int c{};
static_assert(a == 5 && b == 5 && c == 0);
