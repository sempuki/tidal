// tidal-rules: PACK-13
// tidal-expect: accept
// tidal-note: Each copy of the pattern initializes one member. The running total shows that the elements are evaluated from left to right.
struct Three { int a, b, c; };
template <class... T> constexpr Three running(T... x) { int total = 0; return Three{(total += x)...}; }
constexpr Three r = running(1, 2, 3);
static_assert(r.a == 1 && r.b == 3 && r.c == 6);
template <class... T> constexpr int args(T... x) { return sizeof...(x); }
template <class... T> constexpr int pairs(T... x) { return args((x + 1)..., (x * 2)...); }
static_assert(pairs(1, 2, 3) == 6);
