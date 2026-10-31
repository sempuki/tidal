// tidal-rules: UN-18
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct C { operator int() const { return 1; } };
struct D { operator int() const noexcept { return 1; } };
static_assert(!noexcept(int(C{})));
static_assert(noexcept(int(D{})));
