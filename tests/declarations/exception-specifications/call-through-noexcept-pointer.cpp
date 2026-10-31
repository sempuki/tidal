// tidal-rules: EXS-2
// tidal-expect: accept
void (*p)() noexcept = nullptr;
static_assert(noexcept(p()));
