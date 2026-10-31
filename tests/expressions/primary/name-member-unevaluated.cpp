// tidal-rules: PRIM-12
// tidal-expect: accept
struct S { int v; static constexpr auto n = sizeof(v); };
static_assert(sizeof(S::v) == sizeof(int));
