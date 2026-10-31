// tidal-rules: EXS-4
// tidal-expect: accept
void f() {}
static_assert(!noexcept(f()));
