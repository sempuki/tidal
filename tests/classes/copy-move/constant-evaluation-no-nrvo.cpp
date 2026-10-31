// tidal-rules: CPY-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { int moves = 0; constexpr S() {} constexpr S(S&& o) : moves(o.moves + 1) {} constexpr S(const S& o) : moves(o.moves) {} };
constexpr S make() { S s; return s; }
static_assert(make().moves == 1);
