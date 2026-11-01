// tidal-rules: PACK-12
// tidal-expect: accept
// tidal-note: In void(T..., int) the expansion is not last, so the function type gives T no elements. T is then empty, void(int) does not match void(char, int), and the call chooses the other overload.
template <class... T> constexpr int take(void (*)(T..., int)) { return 1; }
constexpr int take(...) { return 2; }
void pair_of(char, int);
static_assert(take(pair_of) == 2);
