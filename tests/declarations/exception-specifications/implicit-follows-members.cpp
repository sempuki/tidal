// tidal-rules: EXS-4
// tidal-expect: accept
struct M { M() {} M(const M&) {} };
struct S { M m; };
static_assert(!noexcept(S()));
static_assert(!noexcept(S(S())));
