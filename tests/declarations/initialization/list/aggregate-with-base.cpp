// tidal-rules: LIST-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct D : B { int d; };
constexpr D x{{1}, 2};
constexpr D y{1, 2};
static_assert(x.b == 1 && x.d == 2 && y.b == 1 && y.d == 2);
