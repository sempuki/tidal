// tidal-rules: EXS-4
// tidal-expect: accept
constexpr int f() { return 1; }
static_assert(!noexcept(f()));
