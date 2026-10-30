// tidal-rules: LOG-7
// tidal-expect: accept
int runtime_only();
constexpr bool h() { return false && runtime_only(); }
constexpr bool k() { return true || runtime_only(); }
static_assert(!h() && k());
