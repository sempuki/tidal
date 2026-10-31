// tidal-rules: BF-4
// tidal-expect: accept
struct S { int x : 3; };
auto n = sizeof(int);
S s{};
auto v = s.x;
static_assert(sizeof(v) == sizeof(int));
