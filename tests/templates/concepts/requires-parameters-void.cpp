// tidal-rules: CPT-28
// tidal-expect: accept
constexpr bool empty = requires (void) { 1; };
static_assert(empty);
