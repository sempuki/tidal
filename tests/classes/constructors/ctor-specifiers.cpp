// tidal-rules: CTOR-1
// tidal-expect: accept
struct S {
  constexpr explicit S(int v) : v(v) {}
  inline S() : v(0) {}
  int v;
};
constexpr S s(3);
static_assert(s.v == 3);
