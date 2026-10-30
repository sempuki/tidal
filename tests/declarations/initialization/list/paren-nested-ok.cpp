// tidal-rules: LIST-9
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct In { int a, b; };
struct Out { In i; int c; };
constexpr Out o(In{1, 2}, 3);
static_assert(o.i.b == 2 && o.c == 3);
