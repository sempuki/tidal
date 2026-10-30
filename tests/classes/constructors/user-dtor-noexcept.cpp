// tidal-rules: DTOR-4
// tidal-expect: accept
struct S { ~S() {} };
static_assert(noexcept(S().~S()));
