// tidal-rules: LIST-4
// tidal-expect: accept
constexpr int a[2][2] = {1, 2, 3, 4};
static_assert(a[1][0] == 3);
struct P { int x; int y; };
struct L { P from; P to; };
constexpr L l = {1, 2, 3, 4};
static_assert(l.to.x == 3);
struct A { int v[2]; int w; };
constexpr A s = {1, 2, 3};
static_assert(s.w == 3);
