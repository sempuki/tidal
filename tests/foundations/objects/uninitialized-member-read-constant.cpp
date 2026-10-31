// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-gcc: accessing uninitialized member .P::y
struct P { int x; int y; constexpr P(int v) : x(v) {} };
constexpr int f() { P p(1); return p.y; }
static_assert(f() == 0);
