// tidal-rules: EXS-4
// tidal-expect: accept
struct S { int v; S() noexcept = default; };
static_assert(noexcept(S()));
