// tidal-rules: DED-12
// tidal-expect: accept
// tidal-note: The types match apart from top-level const, so deduction succeeds.
template <int N> struct Fixed {};
template <const int N> constexpr int take(Fixed<N>) { return N; }
static_assert(take(Fixed<4>{}) == 4);
