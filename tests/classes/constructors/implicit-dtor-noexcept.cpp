// tidal-rules: DTOR-4
// tidal-expect: accept
struct S { int v; };
static_assert(noexcept(S().~S()));
