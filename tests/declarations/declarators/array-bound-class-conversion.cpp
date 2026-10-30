// tidal-rules: DCL-5
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct Size { constexpr operator int() const { return 3; } };
constexpr Size n{};
int a[n];
static_assert(sizeof(a) == 3 * sizeof(int));
