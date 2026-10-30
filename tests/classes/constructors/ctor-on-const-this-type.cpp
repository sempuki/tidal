// tidal-rules: CTOR-11
// tidal-expect: accept
struct S {
  int v;
  constexpr S() : v(0) { v = 3; }
};
constexpr S s;
static_assert(s.v == 3);
