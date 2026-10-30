// tidal-rules: PFX-1
// tidal-expect: accept
constexpr int a[3] = {10, 20, 30};
static_assert(a[1] == 20 && *(a + 1) == 20 && 1[a] == 20);
constexpr const int* p = a;
static_assert(p[2] == 30);
