// tidal-rules: BF-6
// tidal-expect: accept
struct S { unsigned short x : 3; };
S s{};
auto v = s.x;
static_assert(sizeof(v) == sizeof(unsigned short));
