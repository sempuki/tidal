// tidal-rules: LIST-5
// tidal-expect: accept
struct S { int a; int b; int c; };
constexpr S s{.a = 1, .c = 3};
static_assert(s.a == 1 && s.b == 0 && s.c == 3);
constexpr S t = {.b = 2};
static_assert(t.a == 0 && t.b == 2);
