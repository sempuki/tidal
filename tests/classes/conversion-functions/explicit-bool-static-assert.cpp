// tidal-rules: CVF-3
// tidal-expect: accept
struct H { constexpr explicit operator bool() const { return true; } };
static_assert(H{});
