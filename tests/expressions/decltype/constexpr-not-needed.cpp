// tidal-rules: UNEV-1
// tidal-expect: accept
int runtime();
constexpr auto n = sizeof(runtime());
static_assert(n == sizeof(int));
