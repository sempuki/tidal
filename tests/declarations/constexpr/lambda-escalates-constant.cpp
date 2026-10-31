// tidal-rules: CEV-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
consteval int twice(int v) { return v * 2; }
auto call = [](int v) { return twice(v); };
constexpr int r = call(3);
static_assert(r == 6);
