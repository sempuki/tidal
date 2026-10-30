// tidal-rules: LIST-3
// tidal-expect: accept
struct S { int a; double b; char c; };
constexpr S s{1, 2.5, 3};
static_assert(s.a == 1 && s.b == 2.5 && s.c == 3);
