// tidal-rules: EXS-4
// tidal-expect: accept
struct M { M() noexcept(false) {} };
struct S { M m; S() = default; };
static_assert(!noexcept(S()));
