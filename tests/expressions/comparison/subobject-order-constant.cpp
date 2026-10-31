// tidal-rules: CMP-7
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct P { int x; int y; };
constexpr P arr[2]{};
static_assert(&arr[0].y < &arr[1].x);
