// tidal-rules: BF-12
// tidal-expect: accept
struct Gap { unsigned x : 4; unsigned : 3; unsigned y : 4; };
constexpr Gap fresh() { Gap g{}; return g; }   // the copy skips the unnamed bit-field
static_assert(fresh().y == 0 && fresh().x == 0);
constexpr Gap plain() { Gap g; g.x = 1; g.y = 2; Gap h = g; return h; }
static_assert(plain().x == 1 && plain().y == 2);
