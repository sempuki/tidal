// tidal-rules: EXS-4
// tidal-expect: accept
int may_throw();
struct S { int v = may_throw(); };
static_assert(!noexcept(S()));
