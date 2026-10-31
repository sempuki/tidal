// tidal-rules: EXS-4
// tidal-expect: accept
struct S { int v; S() = default; };
static_assert(noexcept(S()));
static_assert(noexcept(S(S())));
static_assert(noexcept(S{} = S{}));
