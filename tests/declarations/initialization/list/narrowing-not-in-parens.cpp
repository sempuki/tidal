// tidal-rules: LIST-2
// tidal-expect: accept
struct S { int a; };
constexpr S s(2.5);
static_assert(s.a == 2);
