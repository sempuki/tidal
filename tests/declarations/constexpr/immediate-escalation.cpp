// tidal-rules: CEV-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
consteval int square(int v) { return v * v; }
template <class T> constexpr int forward_square(T v) { return square(v); }
static_assert(forward_square(3) == 9);
