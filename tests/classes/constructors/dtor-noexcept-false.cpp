// tidal-rules: DTOR-4
// tidal-expect: accept
struct S { ~S() noexcept(false) {} };
static_assert(!noexcept(S().~S()));
