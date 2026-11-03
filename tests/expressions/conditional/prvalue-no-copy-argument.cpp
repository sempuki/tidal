// tidal-rules: COND-12
// tidal-expect: accept
// tidal-note: The operands are prvalues of a class with no copy or move constructor, and the result initializes a function parameter.
struct S {
  int v;
  constexpr S(int x) : v(x) {}
  S(const S&) = delete;
  S(S&&) = delete;
};
constexpr int take(S s) { return s.v; }
static_assert(take(true ? S(1) : S(2)) == 1);
static_assert(take(false ? S(1) : S(2)) == 2);
