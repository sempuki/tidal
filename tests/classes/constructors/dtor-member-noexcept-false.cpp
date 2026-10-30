// tidal-rules: DTOR-4
// tidal-expect: accept
struct M { ~M() noexcept(false) {} };
struct S { M m; ~S() {} };
static_assert(!noexcept(S().~S()));
