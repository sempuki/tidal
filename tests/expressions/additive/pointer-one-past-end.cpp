// tidal-rules: ADD-6
// tidal-expect: accept
constexpr int a[3] = {1, 2, 3};
constexpr const int* end = a + 3;
static_assert(end - a == 3);
