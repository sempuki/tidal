// tidal-rules: EXS-4
// tidal-expect: accept
auto l = [] { return 1; };
auto m = [] () noexcept { return 1; };
static_assert(!noexcept(l()));
static_assert(noexcept(m()));
