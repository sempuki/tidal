// tidal-rules: EXS-2
// tidal-expect: accept
struct S { int get() const noexcept { return 1; } int other() const { return 2; } };
static_assert(noexcept(S{}.get()));
static_assert(!noexcept(S{}.other()));
