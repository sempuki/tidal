// tidal-rules: CE-22 CPY-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: not an integral constant expression|must be initialized by a constant expression
// tidal-error-edg: expression must have a constant value
// tidal-note: Q-0115. Decision 0059, item F2 (R10-4), with the round 9 writer probe r10-4-return-local-unset.json. A constexpr function returns a local whose member b has no value, and the caller reads only member a. The move of the returned local reads b. Clang and EDG reject; GCC accepts, for a named local, a parenthesized local, and a local through a conditional.
struct Pair {
  int a;
  int b;
  constexpr Pair() : a(1) {}
};
constexpr Pair make() {
  Pair p;
  return p;
}
static_assert(make().a == 1);
