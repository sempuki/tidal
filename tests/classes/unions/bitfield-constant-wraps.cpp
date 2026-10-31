// tidal-rules: BF-10
// tidal-expect: accept
struct S { unsigned x : 3; };
constexpr S make() { S s{}; s.x = 9; return s; }
static_assert(make().x == 1);
