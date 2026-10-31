// tidal-rules: DCMP-1
// tidal-expect: accept
struct P { int x; int y; bool operator==(const P&) const = default; };
static_assert(P{1, 2} == P{1, 2});
static_assert(!(P{1, 2} == P{1, 3}));
