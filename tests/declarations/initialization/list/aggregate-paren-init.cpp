// tidal-rules: LIST-3
// tidal-expect: accept
struct S { int a; int b; };
constexpr S s(1, 2);
static_assert(s.a == 1 && s.b == 2);
