// tidal-rules: LIST-3
// tidal-expect: accept
struct S { int a; int b = 7; };
constexpr S s{1};
static_assert(s.b == 7);
