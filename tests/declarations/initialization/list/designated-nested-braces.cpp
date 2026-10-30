// tidal-rules: LIST-5
// tidal-expect: accept
struct P { int x; int y; };
struct S { P p; int z; };
constexpr S s{.p = {.y = 2}, .z = 3};
static_assert(s.p.x == 0 && s.p.y == 2 && s.z == 3);
