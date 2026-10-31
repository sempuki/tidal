// tidal-rules: EXS-1
// tidal-expect: accept
void f(int x) noexcept(noexcept(x + 1));
static_assert(noexcept(f(1)));
