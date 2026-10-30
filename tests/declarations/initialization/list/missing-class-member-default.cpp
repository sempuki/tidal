// tidal-rules: LIST-8
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct M { int v = 7; };
struct A { int a; M m; };
constexpr A x{1};
static_assert(x.m.v == 7);
