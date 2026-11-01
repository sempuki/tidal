// tidal-rules: DED-12
// tidal-expect: accept
// tidal-note: P is declared auto*. For Wide<&n> the type taken from Wide's parameter is int*, which has the form auto*. For Wide<5> the type is int, which does not, so the pair fails and the other overload is chosen.
template <auto V> struct Wide {};
template <auto* P> constexpr int kind(Wide<P>) { return 1; }
constexpr int kind(...) { return 2; }
int n = 0;
static_assert(kind(Wide<&n>{}) == 1);
static_assert(kind(Wide<5>{}) == 2);
