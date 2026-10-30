// tidal-rules: LOG-7
// tidal-expect: reject
int runtime_only();
constexpr bool h() { return true && runtime_only(); }
static_assert(h());
