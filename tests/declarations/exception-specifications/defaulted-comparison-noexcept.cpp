// tidal-rules: EXS-4
// tidal-expect: accept
struct S { int a; bool operator==(const S&) const = default; };
static_assert(noexcept(S{} == S{}));
