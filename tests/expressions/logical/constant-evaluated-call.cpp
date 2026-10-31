// tidal-rules: LOG-7
// tidal-expect: reject
// tidal-error-gcc: call to non-.constexpr. function
int runtime_only();
constexpr bool h() { return true && runtime_only(); }
static_assert(h());
