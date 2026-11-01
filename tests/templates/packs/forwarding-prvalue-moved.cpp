// tidal-rules: PACK-23
// tidal-expect: accept
// tidal-note: A direct call initializes the parameter from the prvalue, with no move. A forwarded call binds the prvalue to the reference x first, so the parameter is initialized by the move constructor.
struct Moves {
  int n = 0;
  constexpr Moves() {}
  constexpr Moves(Moves&& other) : n(other.n + 1) {}
};
constexpr int direct(Moves m) { return m.n; }
template <class A> constexpr int relay(A&& x) { return direct(static_cast<A&&>(x)); }
static_assert(direct(Moves{}) == 0);
static_assert(relay(Moves{}) == 1);
