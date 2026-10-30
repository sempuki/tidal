// tidal-rules: PFX-1
// tidal-expect: accept
enum Index { First, Second };
constexpr int a[2] = {5, 6};
static_assert(a[Second] == 6);
