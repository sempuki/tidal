// tidal-rules: CEV-1
// tidal-expect: accept
consteval int square(int v) { return v * v; }
consteval int quad(int v) { return square(square(v)); }
static_assert(quad(2) == 16);
