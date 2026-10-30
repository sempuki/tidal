// tidal-rules: ADD-6
// tidal-expect: accept
constexpr int x = 5;
constexpr const int* end = &x + 1;
static_assert(end - &x == 1);
