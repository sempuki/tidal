// tidal-rules: ADD-6
// tidal-expect: accept
constexpr int a[4] = {1, 2, 3, 4};
constexpr const int* p = a + 2;
static_assert(*p == 3);
static_assert(*(p - 2) == 1);
static_assert(*(1 + a) == 2);
